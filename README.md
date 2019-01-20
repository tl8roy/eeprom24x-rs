# Rust 24x EEPROM Driver

[![crates.io](https://img.shields.io/crates/v/eeprom24x.svg)](https://crates.io/crates/eeprom24x)
[![Docs](https://docs.rs/eeprom24x/badge.svg)](https://docs.rs/eeprom24x)
[![Build Status](https://travis-ci.org/eldruin/eeprom24x-rs.svg?branch=master)](https://travis-ci.org/eldruin/eeprom24x-rs)
[![Coverage Status](https://coveralls.io/repos/eldruin/eeprom24x-rs/badge.svg?branch=master)](https://coveralls.io/r/eldruin/eeprom24x-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform agnostic Rust driver for the 24x series serial EEPROM,
based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Read a single byte from a memory address. See: `read_byte()`.
- Read a byte array starting on a memory address. See: `read_data()`.
- Read the current memory address (please read notes). See: `read_current_address()`.
- Write a byte to a memory address. See: `write_byte()`.
- Write a byte array (up to a memory page) to a memory address. See: `write_page()`.

Can be used at least with the devices listed below.

[Introductory blog post](https://blog.eldruin.com/24x-serial-eeprom-driver-in-rust/)

## The devices
These devices provides a number of bits of serial electrically erasable and
programmable read only memory (EEPROM) organized as a number of words of 8 bits
each. The devices' cascadable feature allows up to 8 devices to share a common
2-wire bus. The devices are optimized for use in many industrial and commercial
applications where low power and low voltage operation are essential.

| Device | Memory bits | 8-bit words | Page size | Datasheet  |
|-------:|------------:|------------:|----------:|:-----------|
|  24x00 |    128 bits |          16 |       N/A | [24C00]    |
|  24x01 |      1 Kbit |         128 |   8 bytes | [AT24C01]  |
|  24x02 |      2 Kbit |         256 |   8 bytes | [AT24C02]  |
|  24x32 |     32 Kbit |       4,096 |  32 bytes | [AT24C32]  |
|  24x64 |     64 Kbit |       8,192 |  32 bytes | [AT24C64]  |
| 24x128 |    128 Kbit |      16,384 |  64 bytes | [AT24C128] |
| 24x256 |    256 Kbit |      32,768 |  64 bytes | [AT24C256] |
| 24x512 |    512 Kbit |      65,536 | 128 bytes | [AT24C512] |

[24C00]: http://ww1.microchip.com/downloads/en/DeviceDoc/24AA00-24LC00-24C00-Data-Sheet-20001178J.pdf
[AT24C01]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8871F-SEEPROM-AT24C01D-02D-Datasheet.pdf
[AT24C02]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8871F-SEEPROM-AT24C01D-02D-Datasheet.pdf
[AT24C32]: http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf
[AT24C64]: http://ww1.microchip.com/downloads/en/devicedoc/doc0336.pdf
[AT24C128]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8734-SEEPROM-AT24C128C-Datasheet.pdf
[AT24C256]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8568-SEEPROM-AT24C256C-Datasheet.pdf
[AT24C512]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8720-SEEPROM-AT24C512C-Datasheet.pdf

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.
In the following examples an instance of the device AT24C256 will be created
as an example. Other devices can be created with similar methods like:
``Eeprom24x::new_24x64(...)``.

```rust
extern crate eeprom24x;
extern crate embedded_hal;
extern crate linux_embedded_hal;

use eeprom24x::{Eeprom24x, SlaveAddr};
use embedded_hal::blocking::delay::DelayMs;
use linux_embedded_hal::{Delay, I2cdev};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut eeprom = Eeprom24x::new_24x256(dev, address);
    let memory_address = [0x12, 0x34];
    let data = 0xAB;

    eeprom.write_byte(memory_address, data).unwrap();

    Delay.delay_ms(5u16);

    let read_data = eeprom.read_byte(memory_address).unwrap();

    println!(
        "Read memory address: [{},{}], retrieved content: {}",
        memory_address[0], memory_address[1], &read_data
    );

    let _dev = eeprom.destroy(); // Get the I2C device back
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

