#![no_std]
#![no_main]
#![allow(async_fn_in_trait)]

//! Raspberry Pi Pico W WiFi Access Point Example
//! This program configures the Pico W as a WiFi access point and creates a TCP server
//! listening on port 1234. It demonstrates basic networking capabilities using the CYW43439
//! wireless chip included on the Pico W.

use core::str::from_utf8;
use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::Duration;
use embedded_io_async::Write;
use rand::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// Bind the PIO0 interrupt to our interrupt handler
// This is required for the WiFi chip's SPI communication
bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

/// Background task that runs the WiFi chip's main loop
/// This task handles the low-level communication with the CYW43 chip
#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

/// Background task that runs the TCP/IP network stack
/// Handles network events, packet processing, and TCP state management
#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting WiFi Access Point...");

    // Initialize the Pico's peripherals
    let p = embassy_rp::init(Default::default());
    let mut rng = RoscRng;

    // Load the WiFi chip's firmware and CLM blob
    // These are required for the CYW43 chip to function
    let fw = include_bytes!("../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../cyw43-firmware/43439A0_clm.bin");

    // Alternative firmware loading method for development:
    // Instead of including firmware in the binary, load from flash memory
    // This makes flashing faster during development
    //     probe-rs download 43439A0.bin --binary-format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs download 43439A0_clm.bin --binary-format bin --chip RP2040 --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    // Configure GPIO pins for WiFi chip communication
    let pwr = Output::new(p.PIN_23, Level::Low);  // Power control
    let cs = Output::new(p.PIN_25, Level::High);  // Chip select
    let mut pio = Pio::new(p.PIO0, Irqs);
    
    // Initialize SPI communication with the WiFi chip
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        p.PIN_24,  // Clock
        p.PIN_29,  // Data
        p.DMA_CH0
    );

    // Initialize the WiFi chip's state
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    
    // Spawn the WiFi background task
    unwrap!(spawner.spawn(cyw43_task(runner)));

    // Initialize the WiFi chip with firmware and power management
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // Configure network settings
    // Using a link-local address (169.254.x.x) for direct communication
    // No DHCP server required with this configuration
    let config = Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: embassy_net::Ipv4Cidr::new(embassy_net::Ipv4Address::new(169, 254, 1, 1), 16),
        dns_servers: heapless::Vec::new(),
        gateway: None,
    });

    // Initialize the network stack with a random seed
    let seed = rng.next_u64();
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed
    );

    // Spawn the network stack background task
    unwrap!(spawner.spawn(net_task(runner)));

    // Start the WiFi access point
    // Can use either open network (start_ap_open) or WPA2 secured (start_ap_wpa2)
    //control.start_ap_open("cyw43", 5).await;
    control.start_ap_wpa2("cyw43", "password", 5).await;

    // Initialize TCP buffers
    let mut rx_buffer = [0; 4096];  // Receive buffer
    let mut tx_buffer = [0; 4096];  // Transmit buffer
    let mut buf = [0; 4096];        // Temporary buffer for data processing

    // Main connection handling loop
    loop {
        // Create a new TCP socket for each connection
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        // Turn off LED while waiting for connection
        control.gpio_set(0, false).await;
        info!("Listening on TCP:1234...");
        
        // Wait for incoming connection
        if let Err(e) = socket.accept(1234).await {
            warn!("accept error: {:?}", e);
            continue;
        }

        // Connection established
        info!("Received connection from {:?}", socket.remote_endpoint());
        control.gpio_set(0, true).await;  // Turn on LED to indicate active connection

        // Handle connection data
        loop {
            // Read data from socket
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    warn!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    warn!("read error: {:?}", e);
                    break;
                }
            };

            // Log received data
            info!("rxd {}", from_utf8(&buf[..n]).unwrap());

            // Echo data back to client
            match socket.write_all(&buf[..n]).await {
                Ok(()) => {}
                Err(e) => {
                    warn!("write error: {:?}", e);
                    break;
                }
            };
        }
    }
}
