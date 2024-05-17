use clap::{Parser, Subcommand};
use crate::{services::server_requests::ReturnedOrganization, utils::process_inputs::process_user_input};


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
        page: u32,
    },
    OrganizationPrograms {
        #[clap(index = 1)]
        index: u32,
    },
}



async fn print_organization(organizations: &Vec<ReturnedOrganization>) {
    loop {
        println!("Please execute a command");
        let args = process_user_input();
        println!("{:?}" , args);
        match OrganizationsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetOrganizationsCommands::Page{page} => {

                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        
                    },
                    GetOrganizationsCommands::OrganizationPrograms{index} => {

                    },
                    // Commands::AllPrograms{limit, page} => {
                    //     if (limit.is_some()) {
                    //         println!("Get valuea: {}", limit.unwrap());
                    //     }
                    //     if (page.is_some()) {
                    //         println!("Get valueb: {}", page.unwrap());
                    //     }
                    // },
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
    }
}

