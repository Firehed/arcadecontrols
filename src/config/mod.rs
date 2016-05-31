
use std::fs::File;
use std::io::Read;
use yaml_rust::{Yaml, YamlLoader};

pub fn load_config(path: String) -> Yaml {
    let mut config = match File::open(path.as_str()) {
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
