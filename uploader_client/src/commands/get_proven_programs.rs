use std::path::Path;

use clap::{Parser, Subcommand};

use crate::{commands::verify_proofs::select_proven_inputs, common, models::returned_program::print_programs_list, services::program_distributor::PagedPrograms, utils::process_inputs::process_user_input};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ProgramsArgs {
    #[command(subcommand)]
    cmd: GetProvenProgramsCommands
}

#[derive(Subcommand, Debug, Clone)]
enum GetProvenProgramsCommands {
    Page {
        #[clap(index = 1)]
        page: usize,
    },
    Verify {
        #[clap(index = 1)]
        index: usize,
    },
}

async fn retrieve_my_proven_programs(limit: Option<usize>, page: Option<usize>) -> PagedPrograms {
    let mut write_guard = common::config::PROGRAM_DISTRIBUTOR_SERVICE.write().expect("Error in rw lock");
    return write_guard.get_my_proven_programs(limit, page).await.expect("Error while getting uploaded programs");
}

// TODO: Update this so that the page size is used
async fn select_proven_program() {
    let mut programs_page = retrieve_my_proven_programs(Some(50), Some(1)).await;

    println!("Proven programs amount: {}", &programs_page.programs.len());
    print_programs_list(&programs_page.programs);

    loop {
        println!("Please execute a command:");
        let args = process_user_input();
        match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    GetProvenProgramsCommands::Page{page} => {
                        programs_page = retrieve_my_proven_programs(Some(50), Some(page)).await;
                    },
                    GetProvenProgramsCommands::Verify{index} => {
                        let chosen_program = &programs_page.programs[index];
                        select_proven_inputs(&chosen_program.program_id, Some(50), Some(1)).await;
                    },
                    // TODO: add here commands for uploaded proofs manipulation
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
        print_programs_list(&programs_page.programs);

    }    
}

pub async fn select_my_proven_programs() {
    select_proven_program().await;
}


