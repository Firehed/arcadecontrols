use i2c;
use i2c::{ Address, Bus };
use mcp23017::MCP23017;
use ini::Ini;


pub struct Config {
    pub address: Address,
    pub bus: Bus,
    pub chip: MCP23017,
}

impl Config {
    fn new(bus: Bus, address: Address) -> Config {
        return Config {
            address: address,
            bus: bus,
            chip: get_chip(bus, address),
        };
    }
}

pub fn from_file(path: &str) -> Vec<Config> {
    let conf = Ini::load_from_file(path).unwrap();

    let bus = conf.general_section().get("bus").unwrap();
    let bus = match bus.parse::<u8>().unwrap() {
        0 => Bus::Dev0,
        _ => Bus::Dev1,
    };

    return (0x20..0x28)
        .map(|i| {
            return (
                i,
                format!("0x{:x}", i)
            );
        })
        .map(|(i, name)| {
            return (
                i,
                conf.section(Some(name).to_owned())
            );
        })
        .filter(|&(_, parsed)| {
            // Remove addresses with no config
            return !parsed.is_none();
        })
        .map(|(i, parsed)| {
            // TODO: actually use the parsed section to set up bindings
            let addr = Address::new(i);
            return Config::new(bus, addr);
        })
        .collect();
}


fn get_chip(bus: Bus, address: Address) -> MCP23017 {
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
