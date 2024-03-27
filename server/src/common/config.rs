use futures_util::lock::Mutex;
use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };

use crate::services::files_storage::aws_s3_handler::AwsS3Handler;
use crate::services::files_storage::file_storage::FileStorage;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
   pub x: i32,
   pub y: i32,
   pub database_url: String,
   pub token: Token,
   pub uploaded_files_url: String, // String that defines where the files are stored, it is a single attribute so that different
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
    pub static ref CONFIG_OBJECT: Config = load_config("./src/config/dev.json").unwrap();
    pub static ref CONNECTION_POOL: Pool<ConnectionManager<MysqlConnection>> = generate_connection_pool(&CONFIG_OBJECT.database_url);
    // pub static ref FILES_STORAGE: AwsS3Handler = AwsS3Handler::new(&CONFIG_OBJECT.uploaded_files_url);
    pub static ref FILES_STORAGE: Mutex<AwsS3Handler> = Mutex::new(AwsS3Handler::new(&CONFIG_OBJECT.uploaded_files_url));
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
    let connection_pool = Pool::builder().test_on_check_out(true).build(manager).expect("Failed to create pool");
    return connection_pool;
}
