use serde_derive::{Serialize, Deserialize};
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::RwLock;
use lazy_static::lazy_static;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };

use crate::common::general_constants;
use crate::common::general_constants::GeneralConstants;
use crate::services::files_storage::aws_s3_handler::AwsS3Handler;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
   pub database_url: String,
   pub token: Token,
   pub files_storage: FilesStorage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub basic_token_secret: String,
    pub basic_token_minutes_duration: u64,
    pub refresh_token_secret: String,
    pub refresh_token_days_duration: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilesStorage {
    pub connection_string: String,
    pub arguments_serarator: String,
}

lazy_static! {
    pub static ref CONFIG_OBJECT: Config = load_config("./src/config/dev.json").unwrap();
    pub static ref CONNECTION_POOL: Pool<ConnectionManager<MysqlConnection>> = generate_connection_pool(&get_database_connection_url(&CONFIG_OBJECT));
    pub static ref FILES_STORAGE: RwLock<AwsS3Handler> = RwLock::new(AwsS3Handler::new(&CONFIG_OBJECT.files_storage));
    pub static ref GENERAL_CONSTANTS: GeneralConstants = general_constants::get_general_constants();
}

fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config_object: Config = serde_json::from_str(&content)?;

    Ok(config_object)
}

fn generate_connection_pool(database_url: &String) -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    
    Pool::builder().test_on_check_out(true).build(manager).expect("Failed to create pool")
}

fn get_database_connection_url(config: &Config) -> String {
    let url_env_variable = env::var("dockerized_database_url");
    if let Ok(ok_env_url) = url_env_variable {
        ok_env_url
    } else {
        config.database_url.clone()
    }
}