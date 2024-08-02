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

    /// Goes back to the previous commands selection
    Back,

    /// Exits the program
    Exit,
}

async fn retrieve_my_proven_programs(limit: usize, page: usize) -> PagedPrograms {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.get_my_proven_programs(Some(limit), Some(page)).await.expect("Error while getting uploaded programs");
}

// TODO: Update this so that the page size is used
pub async fn select_my_proven_programs(first_received_limit: usize, first_received_page: usize) -> bool {
    // let mut should_continue_looping = true;
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut programs_page = retrieve_my_proven_programs(used_limit, used_page).await;
    println!("Proven programs amount: {}", &programs_page.programs.len());
    print_programs_list(&programs_page.programs);

    // while should_continue_looping {
    loop {
        println!("Please execute a command:");
        let args = process_user_input();

        // println!("args: {:?}", args);

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
                    GetProvenProgramsCommands::Back => {
                        // should_continue_looping = false;
                        return true;
                    },
                    GetProvenProgramsCommands::Exit => {
                        return false;
                    },
                    // TODO: add here commands for uploaded proofs manipulation
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


