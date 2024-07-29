use clap::{Parser, Subcommand};
use std::{fs, path::Path, process::Command};
use std::time::{SystemTime, Duration};

use crate::utils::proving::{download_and_run_program, retrieve_programs, run_some_programs};
use crate::{common::{self, communication::EndpointResult}, models::{returned_organization::ReturnedOrganization, returned_program::{print_programs_list, ReturnedProgram}}, services::program_distributor::{PagedPrograms, UploadedProof}, utils::process_inputs::process_user_input};

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
    RunN {
        #[clap(index = 1)]
        amount: usize,
    },
    RunAll,
}

async fn run_all_organization_programs(organization_id: &str) {
    let page_size = common::config::CONFIG_OBJECT.max_page_size;
    let mut programs_page = retrieve_programs(Some(organization_id), Some(page_size), Some(1)).await;
    let mut programs_list = programs_page.programs;

    while programs_list.len() != 0 {
        for returned_program in programs_list {
            download_and_run_program(&returned_program).await;
        }
        programs_page = retrieve_programs(Some(organization_id), Some(page_size), Some(1)).await;
        programs_list = programs_page.programs;
    }

}

pub async fn select_organization_programs(organization_id: &str, limit: usize, first_received_page: usize) {
    let mut programs_page = retrieve_programs(Some(organization_id), Some(limit), Some(first_received_page)).await;
    print_programs_list(&programs_page.programs);

    loop {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page} => {
                        programs_page = retrieve_programs(Some(organization_id), Some(limit), Some(page)).await;
                    },
                    GetProgramsCommands::Run{index} => {
                        let chosen_program = &programs_page.programs[index];
                        download_and_run_program(chosen_program).await;
                    },
                    GetProgramsCommands::RunN{amount} => {
                        run_some_programs(Some(organization_id), amount).await;
                    },
                    GetProgramsCommands::RunAll => {
                        run_all_organization_programs(organization_id).await;
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
