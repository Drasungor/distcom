use serde_derive::{Serialize, Deserialize};
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::RwLock;
use lazy_static::lazy_static;

use crate::services::program_distributor::ProgramDistributorService;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub program_distributor_url: String,
    pub max_page_size : usize,
    pub default_page_size : usize,
}

lazy_static! {
    pub static ref CONFIG_OBJECT: Config = load_config("./src/config/dev.json").unwrap();
    pub static ref PROGRAM_DISTRIBUTOR_SERVICE: RwLock<ProgramDistributorService> = RwLock::new(ProgramDistributorService::new(&get_database_connection_url(&CONFIG_OBJECT)));
}

fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config_object: Config = serde_json::from_str(&content)?;

    Ok(config_object)
}

fn get_database_connection_url(config: &Config) -> String {
    let url_env_variable = env::var("program_distributor_url");
    if let Ok(ok_env_url) = url_env_variable {
        return ok_env_url;
    } else {
        return config.program_distributor_url.clone();
    }
}