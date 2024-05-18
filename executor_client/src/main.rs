// use std::env;
use std::fs::File;
use std::io::Write;
// use clap::{crate_name};
// use clap::{crate_name, Parser, Subcommand};
use clap::{Parser, Subcommand};
// use reqwest::Client;
// use serde_derive::{Deserialize};

use crate::commands::get_organizations::select_organizations;
use crate::services::server_requests::get_organizations;
use crate::utils::compression::decompress_tar;
use crate::utils::process_inputs::process_user_input;
// use serde::de::{DeserializeOwned};

mod common;
mod commands;
mod models;
mod utils;
mod services;



async fn run_program_get_example(program_id: String) {


    let request_url = format!("http://localhost:8080/program/{}", program_id);

    let response = reqwest::get(request_url).await.expect("Error in get");

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        // Open a file to write the downloaded content
        let mut file = File::create("downloaded_file.tar").expect("Error in file creation");
        file.write_all(response.bytes().await.expect("Error in bytes get").as_ref()).expect("Errors in file write");

        decompress_tar("./downloaded_file.tar", "./src/runner/methods");
        
        println!("File downloaded successfully!");
    } else {
        println!("Failed to download file: {}", response.status());
    }
}



async fn get_program_template() {
    let request_url = "http://localhost:8080/program/template";
    let response = reqwest::get(request_url).await.expect("Error in get");

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        // Open a file to write the downloaded content
        let mut file = File::create("downloaded_file.tar").expect("Error in file creation");
        file.write_all(response.bytes().await.expect("Error in bytes get").as_ref()).expect("Errors in file write");

        decompress_tar("./downloaded_file.tar", "./src/runner/methods");
        
        println!("File downloaded successfully!");
    } else {
        println!("Failed to download file: {}", response.status());
    }
}




#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Organizations {
        #[clap(short = 'l', long = "limit")]
        limit: Option<u32>,

        #[clap(short = 'p', long = "page")]
        page: Option<u32>,
    },
    OrganizationPrograms {
        #[clap(short = 'l', long = "limit")]
        limit: Option<u32>,

        #[clap(short = 'p', long = "page")]
        page: Option<u32>,
    },
    AllPrograms {
        #[clap(short = 'l', long = "limit")]
        limit: Option<u32>,

        #[clap(short = 'p', long = "page")]
        page: Option<u32>,
    },
}

async fn run_commands_loop() {
    loop {
        println!("Please execute a command");
        let args = process_user_input();
        println!("{:?}" , args);

        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    Commands::Organizations{limit, page} => {
                        if (limit.is_some()) {
                            println!("Get valuea: {}", limit.unwrap());
                        }
                        if (page.is_some()) {
                            println!("Get valueb: {}", page.unwrap());
                        }
                        select_organizations().await;
                    },
                    Commands::OrganizationPrograms{limit, page} => {
                        if (limit.is_some()) {
                            println!("Get valuea: {}", limit.unwrap());
                        }
                        if (page.is_some()) {
                            println!("Get valueb: {}", page.unwrap());
                        }
                    },
                    Commands::AllPrograms{limit, page} => {
                        if (limit.is_some()) {
                            println!("Get valuea: {}", limit.unwrap());
                        }
                        if (page.is_some()) {
                            println!("Get valueb: {}", page.unwrap());
                        }
                    },
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
    }

}


#[tokio::main]
async fn main() {
    // run_program_get_example("357de710-7ac0-4889-9ce5-6c024db50236".to_string()).await;

    // get_program_template().await;

    // get_program_and_input_group("5793ec0c-d820-4613-bee9-46bf06dd6dbd".to_string()).await;


    // // // compress_folder("../risc_0_examples/basic_prime_test/methods", "./my_compressed_methods.tar").expect("Compression failed");
    // // // compress_folder("./folder_to_compress", "./my_compressed_methods.tar").expect("Compression failed");
    // // compress_folder("./methods", "./my_compressed_methods.tar").expect("Compression failed");

    // compress_folder_contents("./methods_test", "./my_compressed_methods.tar").expect("Compression failed");
    

    // // decompress_tar("./my_compressed_methods.tar", "./src/runner/methods").expect("Decompression failed")
    // decompress_tar("./my_compressed_methods.tar", "./src/runner/methods").expect("Decompression failed")
    // // decompress_tar("./downloaded_file.tar", "./my_decompressed_src").expect("Decompression failed")

    // get_organizations(None, None).await;

    // println!("");
    // println!("");
    // println!("");

    // get_organization_programs("210c3559-86d1-4bbb-999b-dcc1d27867ea".to_string(), None, None).await;

    // println!("");
    // println!("");
    // println!("");


    // get_general_programs(None, None).await;

    run_commands_loop().await;

}