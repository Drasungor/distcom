// // use serde_derive::{Serialize, Deserialize};
// // use std::fs::File;
// // use std::io::Read;
// use lazy_static::lazy_static;
// // use crate::common;
// // use diesel::pg::PgConnection;
// use crate::common::config;
// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// // use diesel::prelude::*;

// lazy_static! {
//     // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     // let connection = PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
//     pub static ref CONNECTION: PgConnection = PgConnection::establish(config::CONFIG_OBJECT.database_url).unwrap_or_else(|_| panic!("Error connecting to {}", config::CONFIG_OBJECT.database_url));
//     // println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaa");
// }


// // fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
// //     let mut file = File::open(path)?;
// //     let mut content = String::new();
// //     file.read_to_string(&mut content)?;
// //     let config_object: Config = serde_json::from_str(&content)?;

// //     Ok(config_object)
// // }
