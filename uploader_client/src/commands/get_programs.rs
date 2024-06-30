use clap::{Parser, Subcommand};
use std::{path::Path, process::Command};

use crate::{common::{self, communication::EndpointResult}, models::returned_program::{print_programs_list, ReturnedProgram}, services::program_distributor::PagedPrograms, utils::process_inputs::process_user_input};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    Page {
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

async fn retrieve_my_programs(limit: Option<usize>, page: Option<usize>) -> PagedPrograms {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.get_my_programs(limit, page).await.expect("Error while getting uploaded programs");
}

async fn post_input_group(program_id: &str, uploaded_input_group_file_path: &Path) {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.upload_input_group(program_id, uploaded_input_group_file_path).await.expect("Error while uploading program input group");
}

// TODO: Update this so that the page size is used
async fn select_program() {
    let mut programs_page = retrieve_my_programs(Some(50), Some(1)).await;
    print_programs_list(&programs_page.programs);

    loop {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page} => {
                        programs_page = retrieve_my_programs(Some(50), Some(page)).await;
                    },
                    GetProgramsCommands::PostInput{index, input_file_path} => {
                        let chosen_program = &programs_page.programs[index];
                        post_input_group(&chosen_program.program_id, Path::new(&input_file_path)).await;
                    },
                    // TODO: add here commands for uploaded proofs manipulation
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
        print_programs_list(&programs_page.programs);

    }    
}

pub async fn select_my_programs() {
    select_program().await;
}


