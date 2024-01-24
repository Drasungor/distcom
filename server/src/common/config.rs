use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
   pub x: i32,
   pub y: i32,
   pub database_url: String,
}

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
