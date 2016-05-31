#[macro_use] extern crate nix;
extern crate yaml_rust;

use std::env;

mod config;
mod mcp23017;
mod i2c;

use i2c::{ Address, Device };
use mcp23017::MCP23017;

fn main() {

    let config: String = match env::args().nth(1) {
        Some(value) => value,
        None => "config.yaml".to_string(),
    };


    let config_yaml = config::load_config(config);

    let device = Device::Dev1;
    let address = Address { a0: false, a1: false, a2: true };
//    let path = "/dev/i2c-1";
    let i2c = match i2c::from_device_and_address(device, address) {
        Err(e) => {
            match e {
                i2c::Error::FileOpenError(x) => println!("Couldn't open i2c: {}", x),
                i2c::Error::IoctlError => println!("No i2c"),
            };
            return;
        },
        Ok(x) => x,
    };

    let mut mcp = MCP23017::new(i2c);

    for i in 1..10 {
        let x = mcp.read();
        println!("x {:?}", x);
    }
}

/*
fn prepare_gpio(mut fd: &File, bus_id: MCP23017) {
    println!("GPIO preparing");
    fd.write_all(b"\x0a\x84");
    fd.write_all(b"\x0a\x00");
    let register = match bus_id {
        MCP23017::GpioA => 0x00,
        MCP23017::GpioB => 0x10,
    };

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
    return buf[0];
}
*/
