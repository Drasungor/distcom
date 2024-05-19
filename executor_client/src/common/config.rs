use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::sync::RwLock;
use lazy_static::lazy_static;

use crate::common::general_constants;
use crate::common::general_constants::GeneralConstants;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
   pub default_limit: usize,
}

lazy_static! {
    pub static ref CONFIG_OBJECT: Config = load_config("./src/config/dev.json").unwrap();
    pub static ref GENERAL_CONSTANTS: GeneralConstants = general_constants::get_general_constants();
}

fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("content: {}", content);

    let config_object: Config = serde_json::from_str(&content)?;

    Ok(config_object)
}
