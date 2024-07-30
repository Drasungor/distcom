use std::path::Path;
use clap::{Parser, Subcommand};
use std::{fs, process::Command};

use crate::{common, models::{returned_input_group::print_input_groups_list, returned_program::print_programs_list}, services::program_distributor::{PagedProgramInputGroups, PagedPrograms}, utils::{process_inputs::{process_previously_set_page_size, process_user_input}, verifying::{retrieve_proven_inputs, verify_all_program_proven_executions, verify_proven_execution, verify_some_program_proven_executions}}};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProofsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProofsCommands {
    Page {
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        #[clap(index = 1)]
        page: usize,
    },
    Verify {
        #[clap(index = 1)]
        index: usize,
    },
    VerifyN {
        #[clap(index = 1)]
        verified_amount: usize,
    },
    VerifyAll,
    Back,
    Exit,
}


pub async fn select_proven_inputs(program_id: &str, first_received_limit: usize, first_received_page: usize) -> bool {
    // let mut should_continue_looping = true;
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut input_groups_page = retrieve_proven_inputs(program_id, used_limit, used_page).await;
    print_input_groups_list(&input_groups_page.program_input_groups);

    // while should_continue_looping {
    loop {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProofsCommands::Page{page, limit} => {
                        used_page = page;
                        used_limit = process_previously_set_page_size(used_limit, limit);
                        // input_groups_page = retrieve_proven_inputs(program_id, used_limit, used_page).await;
                    },
                    GetProofsCommands::Verify{index} => {
                        let chosen_input_group = &input_groups_page.program_input_groups[index];
                        verify_proven_execution(&chosen_input_group.program_id, &chosen_input_group.input_group_id).await;
                        // input_groups_page = retrieve_proven_inputs(program_id, 50, 1).await;
                    },
                    GetProofsCommands::VerifyN {verified_amount} => {
                        verify_some_program_proven_executions(program_id, verified_amount).await;
                        // input_groups_page = retrieve_proven_inputs(program_id, 50, 1).await;
                    },
                    GetProofsCommands::VerifyAll => {
                        verify_all_program_proven_executions(program_id).await;
                        // input_groups_page = retrieve_proven_inputs(program_id, 50, 1).await;
                    },
                    GetProofsCommands::Back => {
                        // should_continue_looping = false;
                        return true;
                    },
                    GetProofsCommands::Exit => {
                        return false;
                    },
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
        };
        // if (should_continue_looping) {
        //     input_groups_page = retrieve_proven_inputs(program_id, used_limit, used_page).await;
        //     print_input_groups_list(&input_groups_page.program_input_groups);
        // }
        input_groups_page = retrieve_proven_inputs(program_id, used_limit, used_page).await;
        print_input_groups_list(&input_groups_page.program_input_groups);
    }    
}
