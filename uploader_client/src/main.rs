use std::path::Path;
use clap::error::ErrorKind;
use commands::get_programs::select_my_programs;
use commands::get_proven_programs::select_my_proven_programs;
use common::communication::AppErrorType;
use services::program_distributor::UploadedProgram;
use clap::{Parser, Subcommand};
use utils::local_storage_helpers::create_folder;
use utils::process_inputs::{process_page_size, process_user_input};

mod services;
mod common;
mod utils;
mod commands;
mod models;

#[derive(Parser)]
#[command(author, version, about, long_about = None, bin_name = "")]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    /// Upload of the methods folder of the code the user wants to upload
    Upload {
        /// Name of the methods folder in the uploads directory
        #[clap(short = 'p', long = "path")]
        folder_name: String,

        /// Name for the program what will be displayed for the provers
        #[clap(short = 'n', long = "name")]
        name: String,

        /// Explanation of the objectives of the program or any additional information
        /// considered necessary
        #[clap(short = 'd', long = "description")]
        description: String,

        /// How many seconds an input group of this program will be blocked before another
        /// prover can request it for execution
        #[clap(short = 't', long = "timeout")]
        execution_timeout: i64,
        
    },

    /// Download of an example of the methods folder for implementation reference
    Template,

    /// Display of the information of the user's uploaded programs,
    /// moves the execution to another commands set
    MyPrograms {

        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },

    /// Display of the user's programs that have at least one input group with a proven execution,
    /// moves the execution to another commands set
    ProvenPrograms {
        
        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },

    /// Exit the program
    Exit,
}

async fn start_program_execution() {
    let mut should_continue_looping = true;
    while should_continue_looping {
        println!();
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Upload{
                        folder_name,
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
                        let folder_path = format!("./uploads/{folder_name}");
                        let upload_methods_result = write_guard.upload_methods(Path::new(&folder_path), uploaded_program_args).await;
                        match upload_methods_result {
                            Ok(program_id) => {
                                let program_folder = format!("./programs_data/{program_id}");
                                create_folder(&program_folder);
                            },
                            Err(received_error) => {
                                if received_error.error_code.parse::<AppErrorType>().unwrap() == AppErrorType::ProgramNameTaken {
                                    println!("Program name is already used by another of your programs");
                                } else {
                                    panic!("Unexpected error while uploading methods: {:?}", received_error);
                                }
                            }
                        }

                    },
                    GetProgramsCommands::Template => {
                        let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
                        let download_template_result = write_guard.download_template_methods(Path::new("./downloads/template")).await;
                        if let Err(received_error) = download_template_result {
                            panic!("Error while downloading template methods: {:?}", received_error);
                        }
                    },
                    GetProgramsCommands::MyPrograms{limit, page} => {
                        let limit_value = process_page_size(limit);
                        should_continue_looping = select_my_programs(limit_value, page).await;
                    },
                    GetProgramsCommands::ProvenPrograms{limit, page} => {
                        let limit_value = process_page_size(limit);
                        should_continue_looping = select_my_proven_programs(limit_value, page).await;
                    },
                    GetProgramsCommands::Exit => {
                        should_continue_looping = false;
                    }
                }
            },
            Err(err) => {
                match err.kind() {
                    ErrorKind::DisplayHelp => {
                        println!("{}", err);
                    },
                    _ => {
                        println!("Invalid command, run the \"help\" command for usage information.")
                    }
                }
            }
       };

    }    
}


#[tokio::main]
async fn main() {
    create_folder("./downloads");
    create_folder("./uploads");
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