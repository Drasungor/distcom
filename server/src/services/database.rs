use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
// use crate::common;
use crate::common::config;

lazy_static! {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let connection = PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    pub static ref connection = PgConnection::establish(config::CONFIG_OBJECT).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaa");

}

pub fn

// fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
//     let mut file = File::open(path)?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;
//     let config_object: Config = serde_json::from_str(&content)?;

//     Ok(config_object)
// }
