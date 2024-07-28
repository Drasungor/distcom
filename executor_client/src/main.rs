use std::fs::File;
use std::io::Write;
use clap::{Parser, Subcommand};
use utils::process_inputs::process_page_size;

use crate::commands::get_organizations::select_organizations;
use crate::commands::get_programs::select_general_programs;
use crate::utils::compression::decompress_tar;
use crate::utils::process_inputs::process_user_input;

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
        limit: Option<usize>,

        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },
    AllPrograms {
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        #[clap(short = 'p', long = "page", default_value = "1")]
        page: Option<u32>,
    },
}

async fn run_commands_loop() {
    loop {
        println!("Please execute a command");
        let args = process_user_input();

        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    Commands::Organizations{limit, page} => {
                        let limit_value = process_page_size(limit);
                        select_organizations().await;
                    },
                    Commands::AllPrograms{limit, page} => {
                        let limit_value = process_page_size(limit);
                        select_general_programs().await;
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
    run_commands_loop().await;

}