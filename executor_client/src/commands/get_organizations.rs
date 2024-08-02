use clap::{error::ErrorKind, Parser, Subcommand};

use crate::{commands::get_organization_programs::select_organization_programs, common, models::returned_organization::print_organizations_list, utils::process_inputs::{process_previously_set_page_size, process_user_input}};

#[derive(Parser)]
#[command(author, version, about, long_about = None, bin_name = "")]
struct OrganizationsArgs {
    #[command(subcommand)]
    cmd: GetOrganizationsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetOrganizationsCommands {
    /// Displays a list with the information of a page of the organizations stored in the program distributor
    Page {
        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(index = 1)]
        page: usize,
    },

    /// Displays the information of the programs stored in the program distributor that belong to the chosen organization
    Choose {
        /// Index of the chosen organization
        #[clap(index = 1)]
        index: usize,
    },

    /// Goes back to the previous commands selection
    Back,

    /// Exits the program
    Exit,
}


pub async fn select_organizations(first_received_limit: usize, first_received_page: usize) -> bool {
    let read_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.read().expect("Error in rw lock");
    let mut used_limit = first_received_limit;
    let mut used_page = first_received_page;
    let mut organizations_page = read_guard.get_organizations(Some(used_limit), Some(used_page)).await;
    print_organizations_list(&organizations_page.organizations);

    loop { 
        println!("Please execute a command");
        let args = process_user_input();

        match OrganizationsArgs::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.cmd {
                    GetOrganizationsCommands::Page{page, limit} => {
                        used_page = page;
                        used_limit = process_previously_set_page_size(used_limit, limit);
                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        organizations_page = read_guard.get_organizations(Some(used_limit), Some(used_page)).await;
                        
                    },
                    GetOrganizationsCommands::Choose{index} => {
                        let chosen_organization = &organizations_page.organizations[index];
                        if !select_organization_programs(&chosen_organization.organization_id, used_limit, 1).await {
                            return false;
                        }
                    },
                    GetOrganizationsCommands::Back => {
                        return true;
                    },
                    GetOrganizationsCommands::Exit => {
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
        print_organizations_list(&organizations_page.organizations);
    }
}