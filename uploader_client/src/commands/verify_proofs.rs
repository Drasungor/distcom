use std::path::Path;
use clap::{Parser, Subcommand};
use std::{fs, process::Command};

use crate::{common, models::{returned_input_group::print_input_groups_list, returned_program::print_programs_list}, services::program_distributor::{PagedProgramInputGroups, PagedPrograms}, utils::process_inputs::process_user_input};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProofsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProofsCommands {
    Page {
        #[clap(index = 1)]
        page: usize,
    },
    Verify {
        #[clap(index = 1)]
        index: usize,
    },
    Back,
}

async fn retrieve_proven_inputs(program_id: &str, limit: Option<usize>, page: Option<usize>) -> PagedProgramInputGroups {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.get_program_proven_inputs(program_id, limit, page).await.expect("Error while getting uploaded programs");
}

async fn verify_proven_execution(program_id: &str, input_group_id: &str) {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.download_program(program_id, Path::new("./src/runner")).await;
    write_guard.download_proof(program_id, input_group_id, Path::new("../downloads/proof.bin")).await.expect("Error while downloading proof");
    // pub async fn download_proof(&mut self, program_id: &str, input_group_id: &str, download_path: &Path) -> Result<(), EndpointError> {

    let output = Command::new("cargo")
        .arg("run")
        // .args(execution_args)
        .current_dir("./src/runner")
        .output()
        .expect("Failed to execute child program");

    println!("Program output: {:?}", output);

}

// TODO: Update this so that the page size is used, do this also with the first page and limit values
async fn select_proven_input(program_id: &str, limit: Option<usize>, page: Option<usize>) {

    let mut should_continue_looping = true;
    let mut input_groups_page = retrieve_proven_inputs(program_id, Some(50), Some(1)).await;
    print_input_groups_list(&input_groups_page.program_input_groups);

    loop {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProofsCommands::Page{page} => {
                        input_groups_page = retrieve_proven_inputs(program_id, Some(50), Some(page)).await;
                    },
                    GetProofsCommands::Verify{index} => {
                        let chosen_input_group = &input_groups_page.program_input_groups[index];
                        verify_proven_execution(&chosen_input_group.program_id, &chosen_input_group.input_group_id).await
                    },
                    GetProofsCommands::Back => {
                        should_continue_looping = false;
                    }
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
       if (should_continue_looping) {
           print_input_groups_list(&input_groups_page.program_input_groups);
       }
    }    
}

pub async fn select_proven_inputs(program_id: &str, limit: Option<usize>, page: Option<usize>) {
    select_proven_input(program_id, limit, page).await;
}
