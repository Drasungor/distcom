use clap::{Parser, Subcommand};
use crate::{models::returned_organization::{print_organizations_list, ReturnedOrganization}, services::server_requests::get_organizations, utils::process_inputs::process_user_input};


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
                    GetOrganizationsCommands::Choose{index} => {

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

async fn select_organizations() {
    let mut organizations_page = get_organizations(Some(50), Some(1)).await;

    loop { 
        println!("Please execute a command");
        let args = process_user_input();
        println!("{:?}" , args);

        print_organizations_list(&organizations_page.data.organizations);

        match OrganizationsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetOrganizationsCommands::Page{page} => {

                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        organizations_page = get_organizations(Some(50), Some(page)).await;
                        
                    },
                    GetOrganizationsCommands::Choose{index} => {
                        let chosen_program = &programs_page.data.programs[index];
                        get_program_and_input_group(&chosen_program.program_id).await;

                        let output = Command::new("cargo")
                            .arg("run")
                            .current_dir("./src/runner")
                            .output()
                            .expect("Failed to execute child program");

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
       print_organizations_list(&organizations_page.data.organizations);
    }
}