#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; 
use rp_pico::hal::{
    clocks::init_clocks_and_plls,
    gpio::Pins,
    i2c::I2C,
    pac,
    sio::Sio,
    usb::UsbBus,
    watchdog::Watchdog,
    fugit::RateExtU32,
};
use embedded_hal_0_2::prelude::{
    _embedded_hal_blocking_i2c_Read,
    _embedded_hal_blocking_i2c_Write
};
use usb_device::prelude::*;
use usbd_serial::SerialPort;
use rp_pico::XOSC_CRYSTAL_FREQ;
use usb_device::class_prelude::UsbBusAllocator;
use heapless::Vec;

// U-blox specific UBX message configuration commands
const UBX_CFG_MSG: &[u8] = &[0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01];
const UBX_CFG_RATE: &[u8] = &[0xB5, 0x62, 0x06, 0x08, 0x06, 0x00, 0x64, 0x00, 0x01, 0x00, 0x01, 0x00];
const GPS_ADDRESS: u8 = 0x42;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .expect("Clocks should initialize correctly");

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS
    );

    // Set up I2C (GPIO16 for SDA and GPIO17 for SCL)
    let mut i2c = I2C::i2c0(
        pac.I2C0,
        pins.gpio16.reconfigure(), // SDA
        pins.gpio17.reconfigure(), // SCL
        400.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    // Set up USB serial
    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(2) // CDC class
        .build();

    // Initial configuration of u-blox module
    // 1. Configure NMEA message output
    i2c.write(GPS_ADDRESS, UBX_CFG_MSG).unwrap();
    
    // 2. Set navigation rate to 10Hz (100ms)
    i2c.write(GPS_ADDRESS, UBX_CFG_RATE).unwrap();

    // Buffer for reading GNSS data
    let mut nmea_buffer: Vec<u8, 256> = Vec::new();

    loop {
        // Poll USB device
        if usb_dev.poll(&mut [&mut serial]) {
            // Attempt to read available bytes from GNSS
            let mut chunk = [0u8; 64];
            match i2c.read(GPS_ADDRESS, &mut chunk) {
                Ok(_) => {
                    // Process received bytes
                    for &byte in chunk.iter().filter(|&&b| b != 0) {
                        // Collect NMEA sentences
                        if byte == b'$' {
                            // Start of a new sentence
                            nmea_buffer.clear();
                        }
                        
                        let _ = nmea_buffer.push(byte);
                        
                        // Check for complete sentence
                        if byte == b'\n' {
                            // Send complete NMEA sentence over USB
                            let _ = serial.write(&nmea_buffer);
                            let _ = serial.flush();
                        }
                    }
                }
                Err(_) => {
                    let _ = serial.write(b"I2C Read Error\n");
                }
            }
        }
    }
}
