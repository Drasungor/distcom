use clap::{Parser, Subcommand};

use crate::{commands::get_organization_programs::select_organization_programs, common, models::returned_organization::print_organizations_list, utils::process_inputs::process_user_input};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct OrganizationsArgs {
    #[command(subcommand)]
    cmd: GetOrganizationsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetOrganizationsCommands {
    Page {
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        // #[clap(short = 'p', long = "page")]
        #[clap(index = 1)]
        page: usize,
    },
    Choose {
        #[clap(index = 1)]
        index: usize,
    },
}


pub async fn select_organizations(limit: usize, first_received_page: usize) {
    let read_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.read().expect("Error in rw lock");
    let mut organizations_page = read_guard.get_organizations(Some(limit), Some(first_received_page)).await;
    let mut used_limit = limit;
    print_organizations_list(&organizations_page.data.organizations);

    loop { 
        println!("Please execute a command");
        let args = process_user_input();

        match OrganizationsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetOrganizationsCommands::Page{page} => {

                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        organizations_page = read_guard.get_organizations(Some(50), Some(page)).await;
                        
                    },
                    GetOrganizationsCommands::Choose{index} => {
                        let chosen_organization = &organizations_page.data.organizations[index];
                        select_organization_programs(&chosen_organization.organization_id).await;
                    },
                }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
        };
        print_organizations_list(&organizations_page.data.organizations);
    }
}