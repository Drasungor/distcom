use clap::{Parser, Subcommand};
use std::process::Command;

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


// async fn download_and_run_program(program: &ReturnedProgram) {
//     let read_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.read().expect("Error in rw lock");

//     let input_file_name = read_guard.get_program_and_input_group(&program.program_id).await;

//     let program_arguments = format!("run {}", input_file_name);

//     let execution_args = vec![input_file_name];

//     println!("program_arguments: {}", program_arguments);

//     let output = Command::new("cargo")
//         .arg("run")
//         .args(execution_args)
//         .current_dir("./src/runner")
//         .output()
//         .expect("Failed to execute child program");

//     println!("Program output: {:?}", output);

// }

async fn retrieve_my_programs(limit: Option<usize>, page: Option<usize>) -> PagedPrograms {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.get_my_programs(limit, page).await.expect("Error while getting uploaded programs");
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
                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        // programs_page = get_organization_programs(&organization.organization_id, Some(50), Some(page)).await;
                        programs_page = retrieve_my_programs(Some(50), Some(1)).await;
                    },
                    GetProgramsCommands::PostInput{index, input_file_path} => {
                        let chosen_program = &programs_page.programs[index];
                        download_and_run_program(chosen_program).await;
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


// pub async fn select_organization_programs(organization: &ReturnedOrganization) {
//     select_program(Some(organization)).await;
// }

pub async fn select_my_programs() {
    select_program().await;
}


