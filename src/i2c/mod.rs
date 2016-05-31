use nix;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Error as IOError};
use std::os::unix::io::AsRawFd;

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

pub fn from_path_and_address(path: &str, address: u16) -> I2CResult {
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
fn set_slave_address(file: &File, slave_address: u16) -> Result<(), nix::Error> {
    // TODO: remove this once back on real HW
    return Ok(());
    let fd = file.as_raw_fd();
    try!(unsafe {
        ioctl_set_i2c_slave_address(fd, slave_address as *mut u8)
    });
    return Ok(());
}