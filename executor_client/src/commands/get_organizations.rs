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
        /// OPTIONAL: Amount displayed
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
    let mut organizations_page_result = read_guard.get_organizations(Some(used_limit), Some(used_page)).await;
    if let Err(organizations_page_err) = organizations_page_result {
        panic!("Error in get organizations: {:?}", organizations_page_err);
    }
    let mut organizations_page = organizations_page_result.unwrap();
    println!("");
    print_organizations_list(&organizations_page.organizations);

    loop { 
        println!("");
        println!("Please execute a command:");
        let args = process_user_input();

        match OrganizationsArgs::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.cmd {
                    GetOrganizationsCommands::Page{page, limit} => {
                        used_page = page;
                        used_limit = process_previously_set_page_size(used_limit, limit);
                    },
                    GetOrganizationsCommands::Choose{index} => {
                        if index < organizations_page.organizations.len() {
                            let chosen_organization = &organizations_page.organizations[index];
                            if !select_organization_programs(&chosen_organization.organization_id, used_limit, 1).await {
                                return false;
                            }
                        } else {
                            println!("Index out of bounds, please choose one of the provided indexes.");
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
        organizations_page_result = read_guard.get_organizations(Some(used_limit), Some(used_page)).await;
        if let Err(organizations_page_err) = organizations_page_result {
            panic!("Error in get organizations: {:?}", organizations_page_err);
        }
        organizations_page = organizations_page_result.unwrap();
        print_organizations_list(&organizations_page.organizations);
    }
}