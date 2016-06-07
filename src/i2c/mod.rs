use nix;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Error as IOError};
use std::os::unix::io::AsRawFd;

// /usr/include/linux/i2c-dev.h
const I2C_SLAVE: u16 = 0x0703;
ioctl!(bad ioctl_set_i2c_slave_address with I2C_SLAVE);

pub struct I2C {
    fh: File,
}

pub enum Error {
    FileOpenError(IOError),
    IoctlError,
}

pub type I2CResult = Result<I2C, Error>;

#[derive(Copy, Clone)]
pub struct Address {
    pub a0: bool,
    pub a1: bool,
    pub a2: bool,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Bus {
    Dev0,
    Dev1,
}

pub fn from_device_and_address(device: Bus, address: Address) -> I2CResult {
    let path = device.to_fs_path();
    let fh = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path);

    let file = match fh {
        Err(e) => return Err(Error::FileOpenError(e)),
        Ok(file) => file,
    };

    match set_slave_address(&file, address) {
        Err(_) => return Err(Error::IoctlError),
        Ok(_) => {},
    }

    return Ok(I2C {
        fh: file,
    });
}

impl I2C {
    pub fn write(&mut self, bytes: &[u8]) {
        let _ = self.fh.write_all(bytes);
    }
    pub fn get_byte(&mut self) -> u8 {
        let mut buf = [0; 1];
        let _ = self.fh.read(&mut buf);
        return buf[0];
    }
}

#[allow(dead_code)]
fn set_slave_address(file: &File, slave_address: Address) -> Result<(), nix::Error> {
    // TODO: remove this once back on real HW
    return Ok(());
    let fd = file.as_raw_fd();
    try!(unsafe {
        ioctl_set_i2c_slave_address(fd, slave_address.as_int() as *mut u8)
    });
    return Ok(());
}

impl Address {
    pub fn new(address: u8) -> Address {
        // Check input range for supported addresses?
        return Address {
            a0: address & 0x01 == 0x01,
            a1: address & 0x02 == 0x02,
            a2: address & 0x04 == 0x04,
        };
    }

    pub fn as_int(&self) -> u8 {
        let mut address = 0x20;
        if self.a0 {
            address = address | 0x01;
        }
        if self.a1 {
            address = address | 0x02;
        }
        if self.a2 {
            address = address | 0x04;
        }
        return address;
    }
}

impl Bus {
    pub fn to_fs_path(&self) -> &str {
        return match *self {
            Bus::Dev0 => "/dev/i2c-0",
            Bus::Dev1 => "/dev/i2c-1",
        };
    }
}
