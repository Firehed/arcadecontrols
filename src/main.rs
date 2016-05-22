#[macro_use] extern crate nix;

use std::os::unix::io::{RawFd, AsRawFd};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write};

// /usr/include/linux/i2c-dev.h
const I2C_SLAVE: u16 = 0x0703;
ioctl!(bad ioctl_set_i2c_slave_address with I2C_SLAVE);

enum MCP23017 {
    GpioA,
    GpioB,
}

fn main() {
    let path = "/dev/i2c-1";
    let fh = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path);

    let file = match fh {
        Err(e) => {
            println!("Oh shit {} {}", path, e);
            return;
        },
        Ok(file) => file,
    };

    let slave_address: u16 = 0x20;

    set_slave_address(&file, slave_address);

    prepare_gpio(&file, MCP23017::GpioA);
    prepare_gpio(&file, MCP23017::GpioB);

    println!("Saw '{}', '{}'",
             read_pin(&file, MCP23017::GpioA),
             read_pin(&file, MCP23017::GpioB));

}

fn prepare_gpio(mut fd: &File, bus_id: MCP23017) {
    println!("GPIO preparing");
    // Write 0x84 to register 0x0A. If the chip is in BANK0 mode, this writes to IOCON to set
    // BANK=1 and ODR=1. If already in BANK1 mode, this writes to OLATA. This also keeps the
    // default value of sequential operation (SEQOP)
    fd.write_all(b"\x0a\x84");
    // Write 0x00 to 0x0A. The above write guarantees being in BANK1, so this always writes to
    // OLATA, resetting it in case the previous write set something.
    fd.write_all(b"\x0a\x00");
    let register = match bus_id {
        MCP23017::GpioA => 0x00,
        MCP23017::GpioB => 0x10,
    };

    // IOCON has SEQOP on from above, so this writes the 7 hardcoded bytes starting at the register
    // determined above (e.g. write first byte to register 0x10, second to 0x11, etc). See Table
    // 1-2 in the MCP23017 spec sheet for addresses (IOCON.BANK=1)
    let seq = [
        register,
        0xFF, // IODIRn   = xFF for all inputs
        0xFF, // IPOLn    = xFF to flip inputs (makes connection to ground = 1)
        0xFF, // GPINTENn = xFF use all pins
        0x00, // DEFVALn  = x00 default value something...?
        0x00, // INTCONn  = x00 compare to previous or default
        0x84, // IOCONn   = x84 like above
        0xFF, // GPPUn    = xFF sets resistors for inputs
    ];
    fd.write_all(&seq);
}

fn read_pin(mut fd: &File, gpio: MCP23017) -> u8 {
    // Table 1.2 from spec sheet, BANK=1
    let address = match gpio {
        MCP23017::GpioA => 0x09,
        MCP23017::GpioB => 0x19,
    };
    fd.write_all(&[address]); // String cast of sorts
    let mut buf = [0;1];
    fd.read(&mut buf);
    return buf_to_i32(&buf);
}

fn buf_to_i32(buf: &[u8]) -> u32 {
    return buf.iter().rev().fold(0, |acc, &b| acc * 2 + b as u32);
}

fn set_slave_address(file: &File, slave_address: u16) -> Result<(), nix::Error> {
    let fd = file.as_raw_fd();
    try!(unsafe {
        ioctl_set_i2c_slave_address(fd, slave_address as *mut u8)
    });
    Ok(())
}
