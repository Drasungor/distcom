use clap::{Parser, Subcommand};

use crate::{services::server_requests::ReturnedProgram, utils::process_inputs::process_user_input};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProgramsCommands {
    Page {
        #[clap(index = 1)]
        page: u32,
    },
    Run {
        #[clap(index = 1)]
        index: u32,
    },
}


async fn select_organization_programs(organizations: &Vec<ReturnedProgram>) {
    loop { 
        println!("Please execute a command");
        let args = process_user_input();
        println!("{:?}" , args);
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page} => {

                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        
                    },
                    GetProgramsCommands::Run{index} => {

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

