use clap::error::ErrorKind;
use clap::{Parser, Subcommand};

use crate::utils::process_inputs::process_previously_set_page_size;
use crate::utils::proving::{download_and_run_program, retrieve_programs, run_some_programs};
use crate::{models::returned_program::print_programs_list, utils::process_inputs::process_user_input};

#[derive(Parser)]
#[command(author, version, about, long_about = None, bin_name = "")]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    /// Displays a list with information of programs regardless of their uploader
    Page {
        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(index = 1)]
        page: usize,
    },

    /// Generates the proof for a specific program run
    Run {
        /// Index of the chosen program
        #[clap(index = 1)]
        index: usize,
    },

    /// Generates the proofs for a bounded amount of programs
    RunN {
        /// Amount of executions that will be proven
        #[clap(index = 1)]
        amount: usize,
    },

    /// Goes back to the previous commands selection
    Back,

    /// Exits the program
    Exit,
}

pub async fn select_general_programs(first_received_limit: usize, first_received_page: usize) -> bool {
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut programs_page = retrieve_programs(None, Some(used_limit), Some(used_page)).await;
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

                        programs_page = retrieve_programs(None, Some(used_limit), Some(used_page)).await;
                    },
                    GetProgramsCommands::Run{index} => {
                        let chosen_program = &programs_page.programs[index];
                        let _ = download_and_run_program(chosen_program).await;
                    },
                    GetProgramsCommands::RunN{amount} => {
                        run_some_programs(None, amount).await;
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
