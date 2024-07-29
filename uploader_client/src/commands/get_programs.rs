use clap::{Parser, Subcommand};
use std::{fs, path::Path, process::Command};

use crate::{common::{self, communication::EndpointResult}, models::returned_program::{print_programs_list, ReturnedProgram}, services::program_distributor::PagedPrograms, utils::{local_storage_helpers::create_folder, process_inputs::{process_previously_set_page_size, process_user_input}}};

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
        input_file_path: String,
    },
}

async fn retrieve_my_programs(limit: usize, page: usize) -> PagedPrograms {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.get_my_programs(Some(limit), Some(page)).await.expect("Error while getting uploaded programs");
}

async fn post_input_group(program_id: &str, uploaded_input_group_file_path: &Path) -> String {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.upload_input_group(program_id, uploaded_input_group_file_path).await.expect("Error while uploading program input group");
}

// TODO: Update this so that the page size is used
pub async fn select_my_programs(first_received_limit: usize, first_received_page: usize) {
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut programs_page = retrieve_my_programs(used_limit, used_page).await;
    print_programs_list(&programs_page.programs);

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
                    GetProgramsCommands::PostInput{index, input_file_path} => {
                        let chosen_program = &programs_page.programs[index];
                        let program_id = &chosen_program.program_id;
                        let input_file_path = Path::new(&input_file_path);
                        let input_group_id = post_input_group(program_id, input_file_path).await;
                        let input_group_folder = format!("./programs_data/{program_id}/{input_group_id}");
                        create_folder(&input_group_folder);
                        let final_input_group_path = format!("{input_group_folder}/{}", input_file_path.file_name().unwrap().to_str().unwrap());
                        fs::copy(input_file_path, final_input_group_path).expect("Error moving input file");
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

