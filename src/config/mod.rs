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
        Config {
            address: address,
            bus: bus,
            chip: get_chip(bus, address),
        }
    }
}

pub fn from_file(path: &str) -> Result<Vec<Config>, &'static str> {
    // The Ini APIs return a mix of Option and Result types. This works its way through the config
    // file, checking that a bus is configured and valid, normalizing errors along the way
    let conf = try!(
        Ini::load_from_file(path)
        .map_err(|_| "Config file not found")
    );
    let bus = try!(
        conf
        .section(None::<String>) // Main section (can't use .general_section because it may panic)
        .ok_or("Main section missing")
        .map(|prop_ref| (*prop_ref).clone()) // &ini::Properties -> ini::Properties
        .and_then(|main| main // look for "bus"
            .get("bus")
            .ok_or("'bus' not found in config")
            .map(|bus_ref| (*bus_ref).clone())
        )
        .and_then(|bus| bus // Convert string value to u8
            .parse::<u8>()
            .map_err(|_| "Bus must be a number"))
        .and_then(|bus_int| match bus_int { // validate range
            0 => Ok(Bus::Dev0),
            1 => Ok(Bus::Dev1),
            _ => Err("Bus must be 0 or 1")
        })
    );


    Ok(
        (0x20..0x28) // MCP27017 addressable range
        .map(|i| (i, format!("0x{:x}", i))) // (i, "0x20")
        .map(|(i, name)| (i, conf.section(Some(name).to_owned()))) // Get sections from file
        .filter(|&(_, parsed)| !parsed.is_none()) // Remove missing
        .map(|(i, parsed)| {
            // TODO: actually use the parsed section to set up bindings
            let addr = Address::new(i);
            Config::new(bus, addr)
        })
        .collect()
    )
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
    MCP23017::new(i2c)
}
