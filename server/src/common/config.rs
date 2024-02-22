use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ ConnectionManager, Pool };


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
   pub x: i32,
   pub y: i32,
   pub database_url: String,
}

lazy_static! {
    pub static ref CONFIG_OBJECT: Config = load_config("./src/config/dev.json").unwrap();
    pub static ref CONNECTION_POOL: Pool<ConnectionManager<MysqlConnection>> = generate_connection_pool(&CONFIG_OBJECT.database_url);
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
