use clap::{error::ErrorKind, Parser, Subcommand};
use std::{fs, path::Path};

use crate::{commands::get_input_groups::select_input_groups, common, models::returned_program::print_programs_list, services::program_distributor::PagedPrograms, utils::{local_storage_helpers::create_folder, process_inputs::{process_previously_set_page_size, process_user_input}}};

#[derive(Parser)]
#[command(author, version, about, long_about = None, bin_name = "")]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    /// Displays a page of the user's uploaded programs
    Page {
        /// OPTIONAL: Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(index = 1)]
        page: usize,
    },

    /// Uploads one or more input groups for a specific program
    PostInput {
        /// Index of the displayed program for which an input group wants to be uploaded
        #[clap(index = 1)]
        index: usize,

        /// Name of the csv file of folder containing csv files inside the uploads folder
        #[clap(index = 2)]
        input_file_name: String,
    },

    /// Gets the selected program input groups,
    /// moves the execution to another commands set
    GetInputs {

        /// Index of the displayed program for which an input group wants to be uploaded
        #[clap(index = 1)]
        index: usize,

        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },

    /// Deletes a program
    Delete {

        /// Index of the displayed program that will be deleted
        #[clap(index = 1)]
        index: usize,
    },

    /// Goes back to the previous commands selection
    Back,

    /// Exits the program
    Exit,
}

async fn retrieve_my_programs(limit: usize, page: usize) -> PagedPrograms {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.get_my_programs(Some(limit), Some(page)).await.expect("Error while getting uploaded programs")
}

async fn post_input_group(program_id: &str, input_group_name: &String, uploaded_input_group_file_path: &Path) -> String {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.upload_input_group(program_id, input_group_name, uploaded_input_group_file_path).await.expect("Error while uploading program input group")
}

async fn delete_program(program_id: &str) {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.delete_program(program_id).await.expect("Error while deleting program");
}


async fn manage_input_group_upload(program_id: &str, uploaded_input_group_file_path: &Path) {
    let file_name = uploaded_input_group_file_path.file_name().unwrap().to_str().unwrap();
    let parts: Vec<&str> = file_name.split('.').collect();
    if uploaded_input_group_file_path.exists() {
        let input_group_id = post_input_group(program_id, &parts[0].to_string(), uploaded_input_group_file_path).await;
        let input_group_folder = format!("./programs_data/{program_id}/{input_group_id}");
        create_folder(&input_group_folder);
        let final_input_group_path = format!("{input_group_folder}/{}", file_name);
        fs::copy(uploaded_input_group_file_path, final_input_group_path).expect("Error moving input file");
        println!("Uploaded input group with path: {}", uploaded_input_group_file_path.to_str().unwrap());
    } else {
        println!("The provided input file does not exist: {}", uploaded_input_group_file_path.to_str().unwrap());
    }
}

async fn upload_inputs_folder(program_id: &str, folder_path: &Path) {
    let dir_entries = fs::read_dir(folder_path).expect("Failed reading the downloads folder");
    for entry in dir_entries {
        let dir_entry = entry.expect("Error in entry parsing");
        let current_path = dir_entry.path();
        let extension_option = current_path.extension();
        if current_path.is_file() && extension_option.is_some() && extension_option.unwrap() == "csv" {
            manage_input_group_upload(program_id, &current_path).await;
        }
    }
} 

pub async fn select_my_programs(first_received_limit: usize, first_received_page: usize) -> bool {
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut programs_page = retrieve_my_programs(used_limit, used_page).await;
    println!();
    print_programs_list(&programs_page.programs);

    loop {
        println!();
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page, limit} => {
                        used_limit = process_previously_set_page_size(used_limit, limit);
                        used_page = page;
                    },
                    GetProgramsCommands::PostInput{index, input_file_name} => {
                        if index < programs_page.programs.len() {
                            let chosen_program = &programs_page.programs[index];
                            let program_id = &chosen_program.program_id;
                            let input_file_path_string = format!("./uploads/{input_file_name}");
                            let input_file_path = Path::new(&input_file_path_string);
                            
                            if input_file_path.is_dir() {
                                upload_inputs_folder(program_id, input_file_path).await;
                            } else {
                                manage_input_group_upload(program_id, input_file_path).await;
                            }
                        } else {
                            println!("Index out of bounds, please choose one of the provided indexes.");
                        }
                    },
                    GetProgramsCommands::GetInputs{index, page, limit} => {
                        let chosen_program = &programs_page.programs[index];
                        let program_id = &chosen_program.program_id;
                        let get_inputs_limit = process_previously_set_page_size(used_limit, limit);
                        if !select_input_groups(program_id, get_inputs_limit, page).await {
                            return false;
                        }
                    },
                    GetProgramsCommands::Delete{index} => {
                        let chosen_program = &programs_page.programs[index];
                        let program_id = &chosen_program.program_id;
                        delete_program(&program_id).await;
                        // We do not delete this program's folder because the user might not want to have the already 
                        // verified inputs removed
                    },
                    GetProgramsCommands::Back => {
                        return true;
                    },
                    GetProgramsCommands::Exit => {
                        return false;
                    },
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
        programs_page = retrieve_my_programs(used_limit, used_page).await;
        print_programs_list(&programs_page.programs);

    }    
}

