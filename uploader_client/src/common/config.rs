use futures_util::lock::Mutex;
use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::sync::RwLock;
use lazy_static::lazy_static;

use crate::services::program_distributor::ProgramDistributorService;

// use diesel::mysql::MysqlConnection;
// use diesel::r2d2::{ ConnectionManager, Pool };

// use crate::common::general_constants;
// use crate::common::general_constants::GeneralConstants;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
   pub database_url: String,
   pub token: Token,
   pub uploaded_files_connection_string: String, // String that defines where the files are stored, it is a single attribute so that different
                                   // parameters can be formatted inside it
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub basic_token_secret: String,
    pub basic_token_minutes_duration: u64,
    pub refresh_token_secret: String,
    pub refresh_token_days_duration: u64,
}

lazy_static! {
    // pub static ref CONFIG_OBJECT: Config = load_config("./src/config/dev.json").unwrap();
    // pub static ref GENERAL_CONSTANTS: GeneralConstants = general_constants::get_general_constants();
    pub static ref PROGRAM_DISTRIBUTOR: ProgramDistributorService = general_constants::get_general_constants();
}

// fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
//     let mut file = File::open(path)?;
//     let mut content = String::new();
//     file.read_to_string(&mut content)?;
//     let config_object: Config = serde_json::from_str(&content)?;

//     Ok(config_object)
// }

fn instantiate_program_distributor_service() -> Result<ProgramDistributorService, Box<dyn std::error::Error>> {

    
    
    Ok(config_object)
}
