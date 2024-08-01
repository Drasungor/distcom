use clap::{Parser, Subcommand};
use std::{fs, path::Path, process::Command};

use crate::{common, models::returned_program::print_programs_list, services::program_distributor::PagedPrograms, utils::{local_storage_helpers::create_folder, process_inputs::{process_previously_set_page_size, process_user_input}}};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    Page {
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        #[clap(index = 1)]
        page: usize,
    },
    PostInput {
        #[clap(index = 1)]
        index: usize,

        #[clap(index = 2)]
        input_file_name: String,
    },
    Back,
    Exit,
}

async fn retrieve_my_programs(limit: usize, page: usize) -> PagedPrograms {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.get_my_programs(Some(limit), Some(page)).await.expect("Error while getting uploaded programs");
}

async fn post_input_group(program_id: &str, uploaded_input_group_file_path: &Path) -> String {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.upload_input_group(program_id, uploaded_input_group_file_path).await.expect("Error while uploading program input group");
}

async fn manage_input_group_upload(program_id: &str, uploaded_input_group_file_path: &Path) {
    let input_group_id = post_input_group(program_id, uploaded_input_group_file_path).await;
    let input_group_folder = format!("./programs_data/{program_id}/{input_group_id}");
    create_folder(&input_group_folder);
    let final_input_group_path = format!("{input_group_folder}/{}", uploaded_input_group_file_path.file_name().unwrap().to_str().unwrap());
    fs::copy(uploaded_input_group_file_path, final_input_group_path).expect("Error moving input file");
    println!("Uploaded input group with path: {}", uploaded_input_group_file_path.to_str().unwrap());
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

// TODO: Update this so that the page size is used
pub async fn select_my_programs(first_received_limit: usize, first_received_page: usize) -> bool {
    // let mut should_continue_looping = true;
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut programs_page = retrieve_my_programs(used_limit, used_page).await;
    print_programs_list(&programs_page.programs);

    // while should_continue_looping {
    loop {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page, limit} => {
                        used_limit = process_previously_set_page_size(used_limit, limit);
                        used_page = page;
                        programs_page = retrieve_my_programs(used_limit, used_page).await;
                    },
                    GetProgramsCommands::PostInput{index, input_file_name} => {
                        let chosen_program = &programs_page.programs[index];
                        let program_id = &chosen_program.program_id;
                        let input_file_path_string = format!("./uploads/{input_file_name}");
                        let input_file_path = Path::new(&input_file_path_string);
                        
                        if input_file_path.is_dir() {
                            upload_inputs_folder(program_id, input_file_path).await;
                        } else {
                            manage_input_group_upload(program_id, input_file_path).await;
                        }
                    },
                    GetProgramsCommands::Back => {
                        // should_continue_looping = false;
                        return true;
                    },
                    GetProgramsCommands::Exit => {
                        // should_continue_looping = false;
                        return false;
                    },
                }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
        };
        print_programs_list(&programs_page.programs);

    }    
}

