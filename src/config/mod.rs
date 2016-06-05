use i2c;
use i2c::{ Address, Device };
use mcp23017::MCP23017;
use std::fs::File;
use std::io::Read;
use yaml_rust::{Yaml, YamlLoader};
use yaml_rust::yaml::Hash;

pub struct Config {
    pub address: Address,
    pub bus: Device,
    pub chip: MCP23017,
}

impl Config {
    fn new(bus: Device, address: Address) -> Config {
        return Config {
            address: address,
            bus: bus,
            chip: get_chip(bus, address),
        };
    }
}

pub fn from_file(path: String) -> Vec<Config> {
    let yaml = parse_config_file(path);
    let mut configs: Vec<Config> = Vec::new();

    // TODO: implement this for all buses
    match yaml["dev1"] {
        Yaml::Hash(ref v) => {
            let mut y = parse_bus(v, Device::Dev1);
            configs.append(&mut y);
        },
        _ => {
            println!("got nothing or wrong");
        }
    };

    return configs;
}

fn parse_config_file(path: String) -> Yaml {
    let mut config = match File::open(path) {
        Err(e) => panic!(e),
        Ok(x) => x,
    };
    let mut content = String::new();
    let _ = match config.read_to_string(&mut content) {
        Err(e) => panic!(e),
        Ok(_) => (),
    };
    let parsed = YamlLoader::load_from_str(content.as_str()).unwrap();
    return parsed[0].clone();
}

fn parse_bus(data: &Hash, bus: Device) -> Vec<Config> {
    // TODO: actually implement this, support all addresses
    vec![
        Config::new(bus, Address { a0: false, a1: false, a2: false }),
        Config::new(bus, Address { a0: true,  a1: true,  a2: true  }),
    ]
}

fn get_chip(bus: Device, address: Address) -> MCP23017 {
    let i2c = match i2c::from_device_and_address(bus, address) {
        Err(e) => {
            match e {
                i2c::Error::FileOpenError(x) => println!("Couldn't open i2c: {}", x),
                i2c::Error::IoctlError => println!("No i2c"),
            };
            panic!("Could not build MCP23017");
        },
        Ok(value) => value,
    };
    return MCP23017::new(i2c);
}
