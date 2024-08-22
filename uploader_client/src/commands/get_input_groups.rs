use clap::{error::ErrorKind, Parser, Subcommand};

use crate::{common, models::returned_input_group::print_input_groups_list, services::program_distributor::PagedProgramInputGroups, utils::process_inputs::{process_previously_set_page_size, process_user_input}};

#[derive(Parser)]
#[command(author, version, about, long_about = None, bin_name = "")]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetInputGroupsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetInputGroupsCommands {
    /// Displays a page of the user's uploaded programs
    Page {
        /// OPTIONAL: Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(index = 1)]
        page: usize,
    },

    /// Deletes an input group
    Delete {
        /// Index of the displayed input group that will be deleted
        #[clap(index = 1)]
        index: usize,
    },

    /// Goes back to the previous commands selection
    Back,

    /// Exits the program
    Exit,
}

async fn retrieve_input_groups(program_id: &str, limit: usize, page: usize) -> PagedProgramInputGroups {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.get_program_input_groups(program_id, Some(limit), Some(page)).await.expect("Error while getting uploaded programs")
}

async fn delete_input_group(program_id: &str, input_group_id: &str) {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    write_guard.delete_input_group(program_id, input_group_id).await.expect("Error while deleting program");
}

pub async fn select_input_groups(program_id: &str, first_received_limit: usize, first_received_page: usize) -> bool {
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut input_groups_page = retrieve_input_groups(program_id, used_limit, used_page).await;
    println!();
    print_input_groups_list(&input_groups_page.program_input_groups);

    loop {
        println!();
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.cmd {
                    GetInputGroupsCommands::Page{page, limit} => {
                        used_limit = process_previously_set_page_size(used_limit, limit);
                        used_page = page;
                    },
                    GetInputGroupsCommands::Delete{index} => {
                        let chosen_input_group = &input_groups_page.program_input_groups[index];
                        let program_id = &chosen_input_group.program_id;
                        let input_group_id = &chosen_input_group.input_group_id;
                        delete_input_group(&program_id, input_group_id).await;
                        // We do not delete this program's folder because the user might not want to have the already 
                        // verified inputs removed
                    },
                    GetInputGroupsCommands::Back => {
                        return true;
                    },
                    GetInputGroupsCommands::Exit => {
                        return false;
                    },
                }
            },
            Err(err) => {
                match err.kind() {
                    ErrorKind::DisplayHelp => {
                        println!("{}", err);
                    },
                    _ => {
                        println!("Invalid command, run the \"help\" command for usage information.")
                    }
                }
            }
        };
        input_groups_page = retrieve_input_groups(program_id, used_limit, used_page).await;
        print_input_groups_list(&input_groups_page.program_input_groups);
    }    
}

