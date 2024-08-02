use clap::error::ErrorKind;
use clap::{Parser, Subcommand};
use utils::process_inputs::process_page_size;

use crate::commands::get_organizations::select_organizations;
use crate::commands::get_programs::select_general_programs;
use crate::utils::process_inputs::process_user_input;

mod common;
mod commands;
mod models;
mod utils;
mod services;

#[derive(Parser)]
#[command(author, version, about, long_about = None, bin_name = "")]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Displays a list with the information of the organizations stored in the program distributor,
    /// moves the execution to another commands set
    Organizations {
        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },

    /// Displays a list with the information of the programs stored in the program distributor without taking
    /// the uploader into account,
    /// moves the execution to another commands set
    AllPrograms {
        /// Amount displayed
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        /// Page number
        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },

    /// Exits the program
    Exit,
}

async fn run_commands_loop() {
    let mut should_continue_looping = true;
    // loop {
    while should_continue_looping {
        println!("Please execute a command");
        let args = process_user_input();

        match Args::try_parse_from(args.iter()) {
            Ok(cli) => {
                match cli.cmd {
                    Commands::Organizations{limit, page} => {
                        let limit_value = process_page_size(limit);
                        should_continue_looping = select_organizations(limit_value, page).await;
                    },
                    Commands::AllPrograms{limit, page} => {
                        let limit_value = process_page_size(limit);
                        should_continue_looping = select_general_programs(limit_value, page).await;
                    },
                    Commands::Exit => {
                        should_continue_looping = false;
                    }
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
    }

}


#[tokio::main]
async fn main() {
    run_commands_loop().await;
}