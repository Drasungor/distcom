use std::env;
use std::fs::File;
use std::io;
use tar::{Builder, Archive};
use clap::{Parser, Subcommand};
use std::process::Command;

use crate::services::server_requests::login;

mod services;
mod common;

fn compress_folder(folder_path: &str, output_path: &str) -> io::Result<()> {
    let file = File::create(output_path)?;
    let mut builder = Builder::new(file);

    // // Recursively add all files in the folder to the tar file
    // builder.append_dir_all(folder_path, folder_path)?;

    // // Recursively add all files in the folder to the tar file
    // let _ = builder.append_dir_all(folder_path, folder_path);

    // Attempt to append all files in the folder to the tar file
    // if let Err(err) = builder.append_dir_all(folder_path, folder_path) {
    if let Err(err) = builder.append_dir_all(folder_path, folder_path) {
        // If an error occurs, call finish to clean up resources and then propagate the error
        let _ = builder.finish();
        return Err(err);
    }

    builder.finish()?;
    Ok(())
}

fn decompress_tar(tar_path: &str, output_folder: &str) -> io::Result<()> {
    let file = File::open(tar_path)?;
    let mut archive = Archive::new(file);

    // archive.unpack(output_folder)?;
    archive.unpack("./")?;

    Ok(())
}

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

// async fn select_program(organization_option: Option<&ReturnedOrganization>) {
//     let mut programs_page = retrieve_programs(organization_option, Some(50), Some(1)).await;
//     print_programs_list(&programs_page.data.programs);

//     loop {
//         println!("Please execute a command:");
//         let args = process_user_input();
//         match ProgramsArgs::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
//             Ok(cli) => {
//                 match cli.cmd {
//                     GetProgramsCommands::Page{page} => {
//                         // get_organization_programs(organization_id: &String, limit: Option<u32>, page: Option<u32>)
//                         // programs_page = get_organization_programs(&organization.organization_id, Some(50), Some(page)).await;
//                         programs_page = retrieve_programs(organization_option, Some(50), Some(page)).await;
//                     },
//                     GetProgramsCommands::Run{index} => {
//                         let chosen_program = &programs_page.data.programs[index];
//                         download_and_run_program(chosen_program).await;
//                     },
//                }
//             }
//             Err(_) => {
//                 println!("That's not a valid command!");
//             }
//        };
//         print_programs_list(&programs_page.data.programs);

//     }    
// }

fn get_input_string() -> String {
    let mut buf = "".to_string();
    std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
    let line = buf.trim();
    return line.to_string();
}

#[tokio::main]
async fn main() {
    println!("Please enter your username:");
    let username = get_input_string();
    
    println!("Please enter your password:");
    let password = get_input_string();
    
    let login_response = login(username, password).await;

    println!("Login response: {:?}", login_response);

}