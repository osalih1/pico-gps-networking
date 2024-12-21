# GPS Communication Project with Raspberry Pi Pico

This project demonstrates different methods of communicating with a GPS module using a Raspberry Pi Pico. The project is structured in branches to show the progression from basic serial communication to WiFi capabilities.

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/osalih1/pico-gps-networking.git
   cd pico-gps-networking
   ```

2. View available branches:
   ```bash
   git branch -a
   ```

   - `main`: Serial communication implementation
   - `wifi`: Experimental WiFi access point setup

3. Switch between branches:
   ```bash
   # For serial implementation
   git checkout main

   # For WiFi implementation
   git checkout wifi
   ```

## Project Structure

### Main Branch
Contains the implementation of serial communication between:
- A Raspberry Pi Pico reading GPS data via I2C
- A laptop interface reading the GPS data through USB serial connection

### WiFi Branch
Contains experimental work towards implementing WiFi communication using the Raspberry Pi Pico W's wireless capabilities. Currently implements:
- Basic WiFi Access Point setup
- TCP server implementation
- Foundation for future UDP-based GPS data transmission

## Hardware Requirements

- Raspberry Pi Pico (for main branch) or Pico W (for WiFi branch)
- U-blox GPS module (I2C interface)
- USB cable for Pico-laptop connection
- Jumper wires for GPS connection

## Pin Connections

### GPS Module to Pico
- SDA → GPIO16
- SCL → GPIO17
- VCC → 3.3V
- GND → GND

## Flashing Instructions

1. **Build the UF2 file:**
   ```bash
   cargo build --release
   ```

2. **Enter BOOTSEL mode on Pico:**
   - Hold BOOTSEL button while connecting USB
   - Pico will appear as a mass storage device

3. **Convert and flash:**
   ```bash
   cargo run
   ```

## Cargo Dependencies

Important note about versions: This project requires specific versions of dependencies to ensure compatibility. Key dependencies:

```toml
[dependencies]
cortex-m = "0.7.7"
cortex-m-rt = "0.7.3"
embedded-hal = "0.2.7"
rp-pico = "0.8"
usb-device = "0.2.9"
usbd-serial = "0.1.1"
```

Using different versions may lead to compilation errors due to:
- Breaking changes in embedded-hal APIs
- Incompatibilities between no_std dependencies
- Changes in Pico SDK bindings

## Project Learnings

### Technical Achievements
1. **Serial Communication Implementation:**
   - Successfully established bidirectional communication between Pico and laptop
   - Implemented NMEA sentence parsing
   - Handled USB serial interface efficiently

2. **WiFi Foundation:**
   - Successfully configured Pico W as an access point
   - Implemented basic TCP server functionality
   - Created groundwork for future UDP implementation

### Key Learning Points
1. **Embedded Rust Development:**
   - Working with no_std environment
   - Understanding embedded HAL traits
   - Managing memory constraints

2. **Hardware Communication:**
   - I2C protocol implementation
   - USB serial interface handling
   - WiFi stack configuration

3. **Development Challenges:**
   - Dependency version management in embedded Rust
   - Async programming in no_std environment
   - Debug limitations in embedded systems

## Future Improvements

1. **WiFi Implementation:**
   - Complete UDP communication for GPS data
   - Implement more robust error handling
   - Add reconnection capabilities

2. **General Enhancements:**
   - Add configuration options for GPS module
   - Implement power management features
   - Add data logging capabilities

## Lessons Learned About Dependencies

The project highlighted the importance of carefully managing Cargo dependencies in embedded Rust projects:

1. **Version Pinning:**
   - Exact versions should be specified for core dependencies
   - Breaking changes are common in embedded ecosystem
   - Updating dependencies requires careful testing

2. **Compatibility Issues:**
   - no_std dependencies require special attention
   - Different HAL versions may have incompatible traits
   - Some crates may not support all target architectures

3. **Best Practices:**
   - Lock dependencies using Cargo.lock
   - Document specific version requirements
   - Test thoroughly when updating any dependency

## License

[Insert your chosen license information here]

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
