use clap::{error::ErrorKind, Parser, Subcommand};

use crate::{models::returned_input_group::print_input_groups_list, utils::{process_inputs::{process_previously_set_page_size, process_user_input}, verifying::{retrieve_proven_inputs, verify_all_program_proven_executions, verify_proven_execution, verify_some_program_proven_executions}}};

#[derive(Parser)]
#[command(author, version, about, long_about = None, bin_name = "")]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProofsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProofsCommands {
    /// Displays a page of the selected program's proven executions
    Page {
        /// OPTIONAL: Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(index = 1)]
        page: usize,
    },

    /// Runs risc zero's proof verification algorithm over the proof associated to the input's execution
    Verify {
        /// Index of the input group whose proof is going to be verified
        #[clap(index = 1)]
        index: usize,
    },

    /// Runs risc zero's proof verification algorithm over a bounded ammount of executions
    VerifyN {
        /// Maximum amount of proofs verified
        #[clap(index = 1)]
        verified_amount: usize,
    },

    /// Runs risc zero's proof verification algorithm over all of this program's proven executions
    VerifyAll,

    /// Goes back to the previous commands selection
    Back,

    /// Exits the program
    Exit,
}


pub async fn select_proven_inputs(program_id: &str, first_received_limit: usize, first_received_page: usize) -> bool {
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut input_groups_page = retrieve_proven_inputs(program_id, used_limit, used_page).await;
    println!("");
    print_input_groups_list(&input_groups_page.program_input_groups);

    loop {
        println!("");
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProofsCommands::Page{page, limit} => {
                        used_page = page;
                        used_limit = process_previously_set_page_size(used_limit, limit);
                    },
                    GetProofsCommands::Verify{index} => {
                        if index < input_groups_page.program_input_groups.len() {
                            let chosen_input_group = &input_groups_page.program_input_groups[index];
                            let _ = verify_proven_execution(&chosen_input_group.program_id, &chosen_input_group.input_group_id).await;
                        } else {
                            println!("Index out of bounds, please choose one of the provided indexes.");
                        }
                    },
                    GetProofsCommands::VerifyN {verified_amount} => {
                        verify_some_program_proven_executions(program_id, verified_amount).await;
                        println!("Finished verifying all programs");
                    },
                    GetProofsCommands::VerifyAll => {
                        verify_all_program_proven_executions(program_id).await;
                        println!("Finished verifying all programs");
                    },
                    GetProofsCommands::Back => {
                        return true;
                    },
                    GetProofsCommands::Exit => {
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
        input_groups_page = retrieve_proven_inputs(program_id, used_limit, used_page).await;
        print_input_groups_list(&input_groups_page.program_input_groups);
    }
}
