//! GPS Serial Port Reader
//! This program reads NMEA sentences from a GPS module connected via USB serial
//! and parses them to extract latitude and longitude coordinates.

use std::ffi::CString;
use std::ptr;
use nmea::Nmea;
use libc::{c_char, c_int, c_void};

// Link to the serialport library for serial communication
#[link(name = "serialport")]
extern "C" {
    // Foreign function declarations for serial port operations
    
    /// Get a serial port handle by its name
    /// Returns 0 on success, non-zero on failure
    fn sp_get_port_by_name(name: *const c_char, port: *mut *mut c_void) -> c_int;
    
    /// Open a serial port with specified flags
    /// Flags: 1 = READ, 2 = WRITE, 3 = READ|WRITE
    fn sp_open(port: *mut c_void, flags: c_int) -> c_int;
    
    /// Close an open serial port
    fn sp_close(port: *mut c_void) -> c_int;
    
    /// Free the memory associated with a port
    fn sp_free_port(port: *mut c_void);
    
    /// Read from serial port with timeout
    /// Returns number of bytes read, 0 on timeout, negative on error
    fn sp_blocking_read(port: *mut c_void, buf: *mut u8, count: usize, timeout_ms: u32) -> c_int;
}

/// Opens a serial port with the given name
/// Returns a handle to the port or an error message
fn open_port(port_name: &str) -> Result<*mut c_void, String> {
    // Convert Rust string to C string for FFI
    let port_name_c = CString::new(port_name).expect("CString::new failed");
    let mut port: *mut c_void = ptr::null_mut();
    
    unsafe {
        // Attempt to get port handle
        if sp_get_port_by_name(port_name_c.as_ptr(), &mut port) != 0 {
            return Err(format!("Failed to get port by name {}", port_name));
        }
        
        // Attempt to open port with read/write access
        if sp_open(port, 3) != 0 {
            sp_free_port(port);
            return Err(format!("Failed to open port {}", port_name));
        }
    }
    Ok(port)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Serial port configuration
    let port_name = "/dev/tty.usbmodem1201";  // USB serial port name
    let port = open_port(port_name)?;
    
    // Initialize NMEA parser and buffers
    let mut nmea_parser = Nmea::default();
    let mut line_buffer = Vec::new();          // Buffer for assembling complete lines
    let mut read_buffer = [0u8; 1024];         // Buffer for raw serial data
    
    // Main reading loop
    loop {
        // Read data from serial port with 1 second timeout
        let bytes_read = unsafe {
            sp_blocking_read(port, read_buffer.as_mut_ptr(), read_buffer.len(), 1000)
        };
        
        // Handle read errors and timeouts
        if bytes_read < 0 {
            eprintln!("Error reading from serial port");
            break;
        } else if bytes_read == 0 {
            // Timeout or no data available
            continue;
        }
        
        // Process the received data
        let data = &read_buffer[..bytes_read as usize];
        for &b in data {
            if b == b'\n' {
                // Complete line received, process it
                let line = String::from_utf8_lossy(&line_buffer);
                let trimmed_line = line.trim();
                
                // Only process valid NMEA sentences (start with '$')
                if trimmed_line.starts_with('$') {
                    match nmea_parser.parse(trimmed_line) {
                        Ok(_) => {
                            // Extract and print coordinates if available
                            if let (Some(lat), Some(lon)) = (nmea_parser.latitude, nmea_parser.longitude) {
                                println!("Lat: {}, Lon: {}", lat, lon);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse NMEA sentence: {:?}. Line: {}", e, trimmed_line);
                        }
                    }
                }
                line_buffer.clear();
            } else if b != b'\r' {
                // Accumulate characters, ignoring carriage returns
                line_buffer.push(b);
            }
        }
    }
    
    // Clean up resources
    unsafe {
        sp_close(port);
        sp_free_port(port);
    }
    Ok(())
}
