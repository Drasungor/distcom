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
}

pub async fn select_general_programs(limit: usize, page: usize) {
    let mut programs_page = retrieve_programs(None, Some(50), Some(1)).await;
    print_programs_list(&programs_page.programs);

    loop {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page} => {
                        programs_page = retrieve_programs(None, Some(50), Some(page)).await;
                    },
                    GetProgramsCommands::Run{index} => {
                        let chosen_program = &programs_page.programs[index];
                        download_and_run_program(chosen_program).await;
                    },
                    GetProgramsCommands::RunN{amount} => {
                        run_some_programs(None, amount).await;
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
