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
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Organizations {
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },
    AllPrograms {
        #[clap(short = 'l', long = "limit")]
        limit: Option<usize>,

        #[clap(short = 'p', long = "page", default_value = "1")]
        page: usize,
    },
    Exit,
}

async fn run_commands_loop() {
    let mut should_continue_looping = true;
    // loop {
    while should_continue_looping {
        println!("Please execute a command");
        let args = process_user_input();

        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
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
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
    }

}


#[tokio::main]
async fn main() {
    run_commands_loop().await;
}