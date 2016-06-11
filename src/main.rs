#[macro_use] extern crate nix;
extern crate ini;

mod config;
mod mcp23017;
mod i2c;

use config::Config;
use std::env;
use std::thread::sleep;

fn main() {

    let config_file: String = match env::args().nth(1) {
        Some(value) => value,
        None => "config.ini".to_string(),
    };

    let mut devices = config::from_file(&config_file);

    // TODO: daemonize or something
    loop {
        for device in &mut devices {
            poll_device(device);
        }
        sleep(std::time::Duration::new(1, 0));
    }
}

fn poll_device(d: &mut Config) {
    // This will read the chip and figure out what keypresses to simpulate depending on the result
    println!("{:?}", d.chip.read());
}
