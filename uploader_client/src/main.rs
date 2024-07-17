use std::env;
use std::fs::File;
use std::path::Path;
use commands::get_programs::select_my_programs;
use commands::get_proven_programs::select_my_proven_programs;
use serde::Serialize;
use services::program_distributor::UploadedProgram;
// use services::program_distributor::{token_refreshment, Token};
use tar::{Builder, Archive};
use clap::{Parser, Subcommand};
use utils::local_storage_helpers::create_folder;
use utils::process_inputs::process_user_input;
use std::process::Command;
use std::io::{self, Read, Write};

use crate::services::program_distributor::ProgramDistributorService;

// use crate::services::program_distributor::login;

mod services;
mod common;
mod utils;
mod commands;
mod models;

// fn compress_folder(folder_path: &str, output_path: &str) -> io::Result<()> {
//     let file = File::create(output_path)?;
//     let mut builder = Builder::new(file);

//     // // Recursively add all files in the folder to the tar file
//     // builder.append_dir_all(folder_path, folder_path)?;

//     // // Recursively add all files in the folder to the tar file
//     // let _ = builder.append_dir_all(folder_path, folder_path);

//     // Attempt to append all files in the folder to the tar file
//     // if let Err(err) = builder.append_dir_all(folder_path, folder_path) {
//     if let Err(err) = builder.append_dir_all(folder_path, folder_path) {
//         // If an error occurs, call finish to clean up resources and then propagate the error
//         let _ = builder.finish();
//         return Err(err);
//     }

//     builder.finish()?;
//     Ok(())
// }

// fn decompress_tar(tar_path: &str, output_folder: &str) -> io::Result<()> {
//     let file = File::open(tar_path)?;
//     let mut archive = Archive::new(file);

//     // archive.unpack(output_folder)?;
//     archive.unpack("./")?;

//     Ok(())
// }

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    Upload {
        #[clap(short = 'p', long = "path")]
        folder_path: String,

        #[clap(short = 'n', long = "name")]
        name: String,

        #[clap(short = 'd', long = "description")]
        description: String,

        #[clap(short = 't', long = "timeout")]
        execution_timeout: i64,
        
    },
    Template,
    MyPrograms {
        #[clap(short = 'l', long = "limit")]
        limit: Option<u32>,

        #[clap(short = 'p', long = "page")]
        page: Option<u32>,
    },
    ProvenPrograms {
        #[clap(short = 'l', long = "limit")]
        limit: Option<u32>,

        #[clap(short = 'p', long = "page")]
        page: Option<u32>,
    },
}

async fn start_program_execution() {
    loop {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Upload{
                        folder_path,
                        name,
                        description,
                        execution_timeout,
                    } => {
                        let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");

                        let uploaded_program_args = UploadedProgram {
                            name,
                            description,
                            execution_timeout,
                        };

                        // TODO: manage this error correctly
                        let program_id = write_guard.upload_methods(Path::new(&folder_path), uploaded_program_args).await.expect("Error in methods upload");
                        let program_folder = format!("./programs_data/{program_id}");
                        create_folder(&program_folder);
                    },
                    GetProgramsCommands::Template => {
                        let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");

                        // TODO: manage this error correctly
                        write_guard.download_template_methods(Path::new("./downloads/template")).await.expect("Error in template download");
                    },
                    GetProgramsCommands::MyPrograms{limit, page} => {
                        if (limit.is_some()) {
                            println!("Get valuea: {}", limit.unwrap());
                        }
                        if (page.is_some()) {
                            println!("Get valueb: {}", page.unwrap());
                        }
                        select_my_programs().await;
                    },
                    GetProgramsCommands::ProvenPrograms{limit, page} => {
                        if (limit.is_some()) {
                            println!("Get valuea: {}", limit.unwrap());
                        }
                        if (page.is_some()) {
                            println!("Get valueb: {}", page.unwrap());
                        }
                        select_my_proven_programs().await;
                    },
                    // pub async fn get_my_proven_programs(&mut self, limit: Option<usize>, page: Option<usize>) -> Result<PagedPrograms, EndpointError>

               }
            }
            Err(err) => {
                println!("That's not a valid command!: {}", err);
            }
       };
        // print_programs_list(&programs_page.data.programs);

    }    
}


#[tokio::main]
async fn main() {
    // let token = interactive_login().await;
    // get_jwt().await;

    create_folder("./downloads");
    create_folder("./aux_files");

    // We create the folder that will store the programs' inputs and outputs
    create_folder("./programs_data");

    {
        // We establish the connection to s3
        let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
        write_guard.setup().await;
    }

    println!("Welcome");

    start_program_execution().await;
}