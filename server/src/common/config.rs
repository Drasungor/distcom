use serde::{Deserialize};
use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    x: i32,
    y: i32,
}


// lazy_static! {
//     pub static ref configObject: Config = load_config("./src/config/dev.json").unwrap();
// }

// pub static ref configObject: Config;


// pub static CONFIG_OBJECT: Config;

// lazy_static! {
//     CONFIG_OBJECT = load_config("./src/config/dev.json").unwrap();
// }

lazy_static! {
    pub static ref CONFIG_OBJECT: Config = load_config("./src/config/dev.json").unwrap();
}


fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config_object: Config = serde_json::from_str(&content)?;

    Ok(config_object)
}


// fn ekisd() {
//     Box::new(configObject).as_ref().
// }

pub fn get_config() -> &'static Config {
    &CONFIG_OBJECT
}