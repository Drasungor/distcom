use clap::error::ErrorKind;
use clap::{Parser, Subcommand};

use crate::utils::process_inputs::process_previously_set_page_size;
use crate::utils::proving::{download_and_run_program, retrieve_programs, run_some_program_inputs, run_some_programs};
use crate::{common, models::returned_program::{print_programs_list, ReturnedProgram}, utils::process_inputs::process_user_input};

#[derive(Parser)]
#[command(author, version, about, long_about = None, bin_name = "")]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    /// Displays a list with information of the chosen organization's programs
    Page {
        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(index = 1)]
        page: usize,
    },

    /// Generates the proof of the chosen program's obtained input group
    Run {
        /// Index of the selected program to prove
        #[clap(index = 1)]
        index: usize,
    },

    /// Generates a proof for a bounded amount of program input groups, which may not belong to the same program
    RunN {
        /// Amount of input groups what will be proven for this organization's programs
        #[clap(index = 1)]
        amount: usize,

        /// Index of the program that will be executed, if this value is not provided then the command will be
        /// applied to all of the organization's programs untill the desired executions amount is reached
        #[clap(index = 1)]
        index: Option<usize>,
    },


    RunAll {
        /// Index of the selected program to prove, if no value is provided then all organization's programs are
        /// executed
        #[clap(index = 1)]
        index: Option<usize>,
    },

    /// Goes back to the previous commands selection
    Back,

    /// Exits the program
    Exit,
}

async fn run_all_organization_programs(organization_id: &str) {
    let page_size = common::config::CONFIG_OBJECT.max_page_size;
    let mut page_counter = 1;
    let mut programs_page = retrieve_programs(Some(organization_id), Some(page_size), Some(page_counter)).await;
    let mut programs_list = programs_page.programs;

    while programs_list.len() != 0 {
        for returned_program in programs_list {
            run_all_program_inputs(&returned_program).await;
        }
        page_counter += 1;
        programs_page = retrieve_programs(Some(organization_id), Some(page_size), Some(page_counter)).await;
        programs_list = programs_page.programs;
    }
}

async fn run_all_program_inputs(chosen_program: &ReturnedProgram) {
    let mut keep_executing_program = true;
    while keep_executing_program {
        keep_executing_program = download_and_run_program(chosen_program).await.is_ok();
    }
}

pub async fn select_organization_programs(organization_id: &str, first_received_limit: usize, first_received_page: usize) -> bool {
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut programs_page = retrieve_programs(Some(organization_id), Some(used_limit), Some(used_page)).await;
    println!("");
    print_programs_list(&programs_page.programs);

    loop {
        println!("");
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page, limit} => {
                        used_page = page;
                        used_limit = process_previously_set_page_size(used_limit, limit);
                        programs_page = retrieve_programs(Some(organization_id), Some(used_limit), Some(used_page)).await;
                    },
                    GetProgramsCommands::Run{index} => {
                        let chosen_program = &programs_page.programs[index];
                        let _ = download_and_run_program(chosen_program).await;
                    },
                    GetProgramsCommands::RunN{amount, index} => {
                        if let Some(index_value) = index {
                            let chosen_program = &programs_page.programs[index_value];
                            run_some_program_inputs(chosen_program, amount).await;
                        }
                        run_some_programs(Some(organization_id), amount).await;
                    },
                    GetProgramsCommands::RunAll{index} => {
                        if let Some(index_value) = index {
                            let chosen_program = &programs_page.programs[index_value];
                            run_all_program_inputs(chosen_program).await;
                        } else {
                            run_all_organization_programs(organization_id).await;
                        }
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
                        println!("{}", err.to_string());
                    },
                    _ => {
                        println!("Invalid command, run the \"help\" command for usage information.")
                    }
                }
            }
        };
        print_programs_list(&programs_page.programs);
    }    
}
