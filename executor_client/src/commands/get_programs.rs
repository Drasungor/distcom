use clap::{Parser, Subcommand};
use std::process::Command;

use crate::{common::communication::EndpointResult, models::{returned_organization::ReturnedOrganization, returned_program::{print_programs_list, ReturnedProgram}}, services::server_requests::{get_general_programs, get_organization_programs, get_program_and_input_group, PagedPrograms}, utils::process_inputs::process_user_input};

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
    Run {
        #[clap(index = 1)]
        index: usize,
    },
}


async fn download_and_run_program(program: &ReturnedProgram) {
    let input_file_name = get_program_and_input_group(&program.program_id).await;

    let program_arguments = format!("run {}", input_file_name);

    let execution_args = vec![input_file_name];

    println!("program_arguments: {}", program_arguments);

    let output = Command::new("cargo")
        .arg("run")
        .args(execution_args)
        .current_dir("./src/runner")
        .output()
        .expect("Failed to execute child program");

    println!("Program output: {:?}", output);

}

async fn retrieve_programs(organization_option: Option<&ReturnedOrganization>, limit: Option<usize>, page: Option<usize>) -> EndpointResult<PagedPrograms> {
    if (organization_option.is_some()) {
        let organization = organization_option.unwrap();
        return get_organization_programs(&organization.organization_id, limit, page).await;
    } else {
        return get_general_programs(limit, page).await;
    }
}

async fn select_program(organization_option: Option<&ReturnedOrganization>) {
    let mut programs_page = retrieve_programs(organization_option, Some(50), Some(1)).await;
    print_programs_list(&programs_page.data.programs);

    loop { 
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page} => {
                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        // programs_page = get_organization_programs(&organization.organization_id, Some(50), Some(page)).await;
                        programs_page = retrieve_programs(organization_option, Some(50), Some(page)).await;
                    },
                    GetProgramsCommands::Run{index} => {
                        let chosen_program = &programs_page.data.programs[index];
                        download_and_run_program(chosen_program).await;
                    },
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
        print_programs_list(&programs_page.data.programs);

    }    
}


pub async fn select_organization_programs(organization: &ReturnedOrganization) {
    select_program(Some(organization)).await;
}

pub async fn select_general_programs() {
    select_program(None).await;
}


