use std::path::Path;

use clap::{error::ErrorKind, Parser, Subcommand};

use crate::{commands::verify_proofs::select_proven_inputs, common, models::returned_program::print_programs_list, services::program_distributor::PagedPrograms, utils::process_inputs::{process_previously_set_page_size, process_user_input}};

#[derive(Parser)]
#[command(author, version, about, long_about = None, bin_name = "")]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProvenProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProvenProgramsCommands {
    /// Displays a page of the user's uploaded programs with at least one proven execution
    Page {
        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(index = 1)]
        page: usize,
    },

    /// Displays the information of a page of the proven input groups of the selected program,
    /// moves the execution to another commands set
    Verify {
        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,

        /// Index of the selected program
        #[clap(index = 1)]
        index: usize,
    },

    /// Verify a bounded amount of executions for programs uploaded
    VerifyN {
        /// Maximum amount of proofs verified
        #[clap(index = 1)]
        verified_amount: usize,
    },

    /// Verify a bounded all executions for programs uploaded
    VerifyAll,

    /// Goes back to the previous commands selection
    Back,

    /// Exits the program
    Exit,
}


pub async fn select_my_proven_programs(first_received_limit: usize, first_received_page: usize) -> bool {
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut programs_page = retrieve_my_proven_programs(used_limit, used_page).await;
    println!("");
    print_programs_list(&programs_page.programs);

    loop {
        println!("");
        println!("Please execute a command:");
        let args = process_user_input();

        match ProgramsArgs::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProvenProgramsCommands::Page{page, limit} => {
                        used_page = page;
                        used_limit = process_previously_set_page_size(used_limit, limit);
                        programs_page = retrieve_my_proven_programs(used_limit, used_page).await;
                    },
                    GetProvenProgramsCommands::Verify{index, limit, page} => {
                        used_page = page;
                        used_limit = process_previously_set_page_size(used_limit, limit);
                        let chosen_program = &programs_page.programs[index];
                        if !select_proven_inputs(&chosen_program.program_id, used_limit, used_page).await {
                            return false;
                        }
                    },
                    GetProvenProgramsCommands::VerifyN {verified_amount} => {

                    }
                    GetProvenProgramsCommands::Back => {
                        return true;
                    },
                    GetProvenProgramsCommands::Exit => {
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


