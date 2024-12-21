//! Raspberry Pi Pico GPS Reader
//! This program interfaces with a U-blox GPS module via I2C and forwards
//! NMEA sentences to a computer via USB serial connection.

#![no_std]  // Embedded system without standard library
#![no_main] // No standard main function entry point
use cortex_m_rt::entry;
use panic_halt as _; // Handle panics by halting

// Import required HAL components for Raspberry Pi Pico
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

// Import I2C traits for communication
use embedded_hal_0_2::prelude::{
    *embedded*hal_blocking_i2c_Read,
    *embedded*hal_blocking_i2c_Write
};

// USB communication components
use usb_device::prelude::*;
use usbd_serial::SerialPort;
use rp_pico::XOSC_CRYSTAL_FREQ;
use usb_device::class_prelude::UsbBusAllocator;
use heapless::Vec;  // Fixed-size vector for no_std environment

// U-blox GPS Configuration Messages
// These are UBX protocol messages to configure the GPS module
const UBX_CFG_MSG: &[u8] = &[0xB5, 0x62, 0x06, 0x01, 0x08, 0x00, 0xF0, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01];
const UBX_CFG_RATE: &[u8] = &[0xB5, 0x62, 0x06, 0x08, 0x06, 0x00, 0x64, 0x00, 0x01, 0x00, 0x01, 0x00];
const GPS_ADDRESS: u8 = 0x42;  // I2C address of the GPS module

#[entry]
fn main() -> ! {
    // Initialize core peripherals
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Initialize system and USB clocks
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

    // Initialize Single-cycle I/O (SIO) block
    let sio = Sio::new(pac.SIO);

    // Initialize GPIO pins
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS
    );

    // Configure I2C with:
    // - GPIO16 as SDA (data line)
    // - GPIO17 as SCL (clock line)
    // - 400kHz clock speed
    let mut i2c = I2C::i2c0(
        pac.I2C0,
        pins.gpio16.reconfigure(), // SDA
        pins.gpio17.reconfigure(), // SCL
        400.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    // Initialize USB serial communication
    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Create USB serial port device
    let mut serial = SerialPort::new(&usb_bus);

    // Configure USB device with VID/PID and CDC class for serial communication
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(2) // CDC class for serial communication
        .build();

    // Configure the GPS module
    // 1. Set up NMEA message output format
    i2c.write(GPS_ADDRESS, UBX_CFG_MSG).unwrap();
    
    // 2. Configure update rate to 10Hz (100ms intervals)
    i2c.write(GPS_ADDRESS, UBX_CFG_RATE).unwrap();

    // Buffer for storing NMEA sentences
    let mut nmea_buffer: Vec<u8, 256> = Vec::new();

    // Main program loop
    loop {
        // Check for USB activity
        if usb_dev.poll(&mut [&mut serial]) {
            // Read data from GPS module
            let mut chunk = [0u8; 64];
            match i2c.read(GPS_ADDRESS, &mut chunk) {
                Ok(_) => {
                    // Process each byte from GPS
                    for &byte in chunk.iter().filter(|&&b| b != 0) {
                        if byte == b'$' {
                            // '$' marks start of new NMEA sentence
                            nmea_buffer.clear();
                        }
                        
                        // NOTE: These lines contain syntax errors and should be:
                        // let _ = nmea_buffer.push(byte);
                        let * = nmea*buffer.push(byte);
                        
                        if byte == b'\n' {
                            // End of NMEA sentence, send over USB
                            // NOTE: This line contains syntax errors and should be:
                            // let _ = serial.write(&nmea_buffer);
                            let * = serial.write(&nmea*buffer);
                            let _ = serial.flush();
                        }
                    }
                }
                Err(_) => {
                    // Report I2C communication errors over USB
                    let _ = serial.write(b"I2C Read Error\n");
                }
            }
        }
    }
}
