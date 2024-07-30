use std::path::Path;
use commands::get_programs::select_my_programs;
use commands::get_proven_programs::select_my_proven_programs;
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
#[command(author, version, about, long_about = None)]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    Upload {
        #[clap(short = 'p', long = "path")]
        // folder_path: String,
        folder_name: String,

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
        limit: Option<usize>,

        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },
    ProvenPrograms {
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },
    Exit,
}

async fn start_program_execution() {
    let mut should_continue_looping = true;
    // loop {
    while should_continue_looping {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Upload{
                        // folder_path,
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

                        let folder_path = format!("{folder_name}");
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