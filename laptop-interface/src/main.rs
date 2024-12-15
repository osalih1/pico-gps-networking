use std::ffi::CString;
use std::ptr;
use nmea::Nmea;
use libc::{c_char, c_int, c_void};

#[link(name = "serialport")]
extern "C" {
    fn sp_get_port_by_name(name: *const c_char, port: *mut *mut c_void) -> c_int;
    fn sp_open(port: *mut c_void, flags: c_int) -> c_int;
    fn sp_close(port: *mut c_void) -> c_int;
    fn sp_free_port(port: *mut c_void);
    fn sp_blocking_read(port: *mut c_void, buf: *mut u8, count: usize, timeout_ms: u32) -> c_int;
}

fn open_port(port_name: &str) -> Result<*mut c_void, String> {
    let port_name_c = CString::new(port_name).expect("CString::new failed");
    let mut port: *mut c_void = ptr::null_mut();

    unsafe {
        if sp_get_port_by_name(port_name_c.as_ptr(), &mut port) != 0 {
            return Err(format!("Failed to get port by name {}", port_name));
        }
        if sp_open(port, 3) != 0 {
            sp_free_port(port);
            return Err(format!("Failed to open port {}", port_name));
        }
    }
    Ok(port)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port_name = "/dev/tty.usbmodem1201";
    let port = open_port(port_name)?;

    let mut nmea_parser = Nmea::default();
    let mut line_buffer = Vec::new();
    let mut read_buffer = [0u8; 1024];

    loop {
        let bytes_read = unsafe {
            sp_blocking_read(port, read_buffer.as_mut_ptr(), read_buffer.len(), 1000)
        };

        if bytes_read < 0 {
            eprintln!("Error reading from serial port");
            break;
        } else if bytes_read == 0 {
            // Timeout or no data
            continue;
        }

        let data = &read_buffer[..bytes_read as usize];
        for &b in data {
            if b == b'\n' {
                // End of line
                let line = String::from_utf8_lossy(&line_buffer);
                let trimmed_line = line.trim();
                
                // Only process lines starting with '$' (valid NMEA sentences)
                if trimmed_line.starts_with('$') {
                    match nmea_parser.parse(trimmed_line) {
                        Ok(_) => {
                            // Check if we have latitude/longitude
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
                line_buffer.push(b);
            }
        }
    }

    unsafe {
        sp_close(port);
        sp_free_port(port);
    }

    Ok(())
}
