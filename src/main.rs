#[macro_use] extern crate nix;
extern crate ini;

mod config;
mod mcp23017;
mod i2c;

use config::Config;
use std::env;

fn main() {

    let config_file: String = match env::args().nth(1) {
        Some(value) => value,
        None => "config.ini".to_string(),
    };

    let mut devices = config::from_file(&config_file);

    // TODO: loop {} or daemonize or something
    // This will become the main application loop
    for i in 0..10 {
        println!("{}", i);
        for device in &mut devices {
            poll_device(device);
        }
    }
}

fn poll_device(d: &mut Config) {
    // This will read the chip and figure out what keypresses to simpulate depending on the result
    println!("{:?}", d.chip.read());
}
