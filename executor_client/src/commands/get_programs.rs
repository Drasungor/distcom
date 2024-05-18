use clap::{Parser, Subcommand};
use std::process::Command;
use std::fs;

use crate::{models::{returned_organization::ReturnedOrganization, returned_program::{print_programs_list, ReturnedProgram}}, services::server_requests::{get_organization_programs, get_program_and_input_group}, utils::process_inputs::process_user_input};

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
        page: usize,
    },
    Run {
        #[clap(index = 1)]
        index: usize,
    },
}


// async fn select_organization_programs(organizations: &Vec<ReturnedProgram>) {
//     loop { 
//         println!("Please execute a command");
//         let args = process_user_input();
//         println!("{:?}" , args);
//         match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
//             Ok(cli) => {
//                 match cli.cmd {
//                     GetProgramsCommands::Page{page} => {

//                         // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        
//                     },
//                     GetProgramsCommands::Run{index} => {
//                         get_program_and_input_group(program_id: String).await;
//                     },
//                     // Commands::AllPrograms{limit, page} => {
//                     //     if (limit.is_some()) {
//                     //         println!("Get valuea: {}", limit.unwrap());
//                     //     }
//                     //     if (page.is_some()) {
//                     //         println!("Get valueb: {}", page.unwrap());
//                     //     }
//                     // },
//                }
//             }
//             Err(_) => {
//                 println!("That's not a valid command!");
//             }
//        };
//     }
// }

pub async fn select_organization_programs(organization: &ReturnedOrganization) {
    let mut programs_page = get_organization_programs(&organization.organization_id, Some(50), Some(1)).await;
    print_programs_list(&programs_page.data.programs);

    loop { 
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProgramsCommands::Page{page} => {

                        // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
                        programs_page = get_organization_programs(&organization.organization_id, Some(50), Some(page)).await;
                        
                    },
                    GetProgramsCommands::Run{index} => {
                        let chosen_program = &programs_page.data.programs[index];
                        let input_file_name = get_program_and_input_group(&chosen_program.program_id).await;

                        let program_arguments = format!("run {}", input_file_name);

                        let output = Command::new("cargo")
                            // .arg("run")
                            .arg(program_arguments)
                            .current_dir("./src/runner")
                            .output()
                            .expect("Failed to execute child program");

                        // fs::remove_dir_all("./program_with_input").expect("Error in program_with_input folder deletion");

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
        print_programs_list(&programs_page.data.programs);

    }
}

