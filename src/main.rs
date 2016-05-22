#[macro_use] extern crate nix;

use std::os::unix::io::{RawFd, AsRawFd};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::{Read, Write};

// /usr/include/linux/i2c-dev.h
const I2C_SLAVE: u16 = 0x0703;
ioctl!(bad ioctl_set_i2c_slave_address with I2C_SLAVE);

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

    prepare_gpio(&file, "A");
    prepare_gpio(&file, "B");

    println!("Saw '{}', '{}'", read_pin(&file, b"\x09"), read_pin(&file, b"\x19"));

}

fn prepare_gpio(mut fd: &File, bus_id: &str) {
    println!("GPIO preparing");
    fd.write_all(b"\x0a\x84");
    fd.write_all(b"\x0a\x00");
    if (bus_id == "A") {
        // addr IODIR IPOL pinmask DEFVAL INTCON IOCON GPPU
        // IODIR = xFF for all inputs
        // IPOL = xFF to flip inputs (makes connection to ground = 1)
        // pinmask(GPINTEN) = xFF use all pins
        // DEFVAL = x00 default value something...?
        // INTCON = x00 compare to previous or default
        // IOCON = x84 ??????
        // GPPU = xFF sets resistors for inputs
        fd.write_all(b"\x00\xff\xff\xff\x00\x00\x84\xff");
    } else if (bus_id == "B") {
        fd.write_all(b"\x10\xff\xff\xff\x00\x00\x84\xff");
    } else {
        panic!("Bad GPIO thing");
    }
}

fn read_pin(mut fd: &File, pin: &[u8]) -> u32 {
    fd.write_all(pin);
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
