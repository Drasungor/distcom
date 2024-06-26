use clap::{Parser, Subcommand};
use crate::{commands::get_programs::select_organization_programs, models::returned_organization::{print_organizations_list, ReturnedOrganization}, services::server_requests::get_organizations, utils::process_inputs::process_user_input};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct OrganizationsArgs {
    #[command(subcommand)]
    cmd: GetOrganizationsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetOrganizationsCommands {
    Page {
        #[clap(index = 1)]
        page: usize,
    },
    Choose {
        #[clap(index = 1)]
        index: usize,
    },
}


pub async fn select_organizations() {
    let mut organizations_page = get_organizations(Some(50), Some(1)).await;
    print_organizations_list(&organizations_page.data.organizations);

    loop { 
        println!("Please execute a command");
        let args = process_user_input();

        match OrganizationsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetOrganizationsCommands::Page{page} => {

                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        organizations_page = get_organizations(Some(50), Some(page)).await;
                        
                    },
                    GetOrganizationsCommands::Choose{index} => {
                        let chosen_organization = &organizations_page.data.organizations[index];
                        select_organization_programs(chosen_organization).await;
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