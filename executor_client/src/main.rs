use std::env;
use std::fs::File;
use std::io::Write;
use std::io;
use std::fs;
use tar::{Builder, Archive};
// use clap::{crate_name};
use clap::{crate_name, Parser, Subcommand};
use reqwest::Client;
use serde_derive::{Deserialize};
use std::process::Command;

use crate::commands::get_organizations::{get_organizations};
use crate::commands::get_programs::{get_organization_programs, get_general_programs};
// use serde::de::{DeserializeOwned};

mod common;
mod commands;
// mod runner;

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

fn compress_folder_contents(folder_path: &str, output_path: &str) -> io::Result<()> {
    let file = File::create(output_path)?;
    let mut builder = Builder::new(file);
    let folder_contents = fs::read_dir(folder_path).expect("Error in ");

    for entry in folder_contents {
        let unwrapped_entry = entry.expect("Error in folder entry processing");
        let path = unwrapped_entry.path();

        let entry_name = unwrapped_entry.file_name().into_string().expect("Error in converion from OsString to string");
        let entry_path = format!("{}/{}", folder_path, entry_name);

        if (path.is_dir()) {
            builder.append_dir_all(format!("./{}", entry_name), entry_path).expect("Error in directory appending");
        } else {
            builder.append_path_with_name(path, entry_name).expect("Error in directory appending");
        }

    }
    builder.finish()?;
    Ok(())
}



fn decompress_tar(tar_path: &str, output_folder: &str) -> io::Result<()> {
    fs::create_dir_all(output_folder)?;
    let file = File::open(tar_path)?;
    let mut archive = Archive::new(file);
    archive.unpack(output_folder)?;
    Ok(())
}


async fn run_program_get_example(program_id: String) {


    let request_url = format!("http://localhost:8080/program/{}", program_id);

    let response = reqwest::get(request_url).await.expect("Error in get");

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        // Open a file to write the downloaded content
        let mut file = File::create("downloaded_file.tar").expect("Error in file creation");
        file.write_all(response.bytes().await.expect("Error in bytes get").as_ref()).expect("Errors in file write");

        decompress_tar("./downloaded_file.tar", "./src/runner/methods");
        
        println!("File downloaded successfully!");
    } else {
        println!("Failed to download file: {}", response.status());
    }
}

async fn get_program_and_input_group(program_id: String) {


    let request_url = format!("http://localhost:8080/program/program-and-inputs/{}", program_id);

    let response = reqwest::get(request_url).await.expect("Error in get");

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        // Open a file to write the downloaded content
        let mut file = File::create("downloaded_program_with_input.tar").expect("Error in file creation");
        file.write_all(response.bytes().await.expect("Error in bytes get").as_ref()).expect("Errors in file write");

        decompress_tar("./downloaded_program_with_input.tar", "./program_with_input");
        

        // We scan the folder for the program .tar file
        let folder_contents = fs::read_dir("./program_with_input").expect("Error in ");

        for entry in folder_contents {

            let unwrapped_entry = entry.expect("Error in folder entry processing");
            let path = unwrapped_entry.path();

            let entry_name = unwrapped_entry.file_name().into_string().expect("Error in converion from OsString to string");
            // let entry_path = format!("{}/{}", folder_path, entry_name);

            // if (path.is_dir()) {
            //     builder.append_dir_all(format!("./{}", entry_name), entry_path).expect("Error in directory appending");
            // } else {
            //     builder.append_path_with_name(path, entry_name).expect("Error in directory appending");
            // }

            let path_string = path.to_str().expect("Error in conversion from path to string");


            // println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAA");
            // println!("path_string: {}", path_string);


            if (entry_name.contains(".tar")) {
                println!("tar path_string: {}", path_string);
                decompress_tar(path_string, "./src/runner/methods");
            }

        }


        // decompress_tar("./downloaded_program_with_input.tar", "./program_with_input");


        let output = Command::new("cargo")
        .arg("run")
        .current_dir("./src/runner")
        .output()
        .expect("Failed to execute child program");


        println!("File downloaded successfully!");
    } else {
        println!("Failed to download file: {}", response.status());
    }
}

async fn get_program_template() {
    let request_url = "http://localhost:8080/program/template";
    let response = reqwest::get(request_url).await.expect("Error in get");

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        // Open a file to write the downloaded content
        let mut file = File::create("downloaded_file.tar").expect("Error in file creation");
        file.write_all(response.bytes().await.expect("Error in bytes get").as_ref()).expect("Errors in file write");

        decompress_tar("./downloaded_file.tar", "./src/runner/methods");
        
        println!("File downloaded successfully!");
    } else {
        println!("Failed to download file: {}", response.status());
    }
}




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
        limit: Option<u32>,

        #[clap(short = 'p', long = "page")]
        page: Option<u32>,
    },
    OrganizationPrograms {
        #[clap(short = 'l', long = "limit")]
        limit: Option<u32>,

        #[clap(short = 'p', long = "page")]
        page: Option<u32>,
    },
    AllPrograms {
        #[clap(short = 'l', long = "limit")]
        limit: Option<u32>,

        #[clap(short = 'p', long = "page")]
        page: Option<u32>,
    },
}

async fn run_commands_loop() {
    loop {
        let mut buf = format!("{} ", crate_name!());
        
        std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
        let line = buf.trim();

        println!("Line value: {}", line);

        let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();

        println!("{:?}" , args);

        match Args::try_parse_from(args.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    Commands::Organizations{limit, page} => {
                        if (limit.is_some()) {
                            println!("Get valuea: {}", limit.unwrap());
                        }
                        if (page.is_some()) {
                            println!("Get valueb: {}", page.unwrap());
                        }
                        // get_organizations(limit, page).await;
                    },
                    Commands::OrganizationPrograms{limit, page} => {
                        if (limit.is_some()) {
                            println!("Get valuea: {}", limit.unwrap());
                        }
                        if (page.is_some()) {
                            println!("Get valueb: {}", page.unwrap());
                        }
                    },
                    Commands::AllPrograms{limit, page} => {
                        if (limit.is_some()) {
                            println!("Get valuea: {}", limit.unwrap());
                        }
                        if (page.is_some()) {
                            println!("Get valueb: {}", page.unwrap());
                        }
                    },
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
    // run_program_get_example("357de710-7ac0-4889-9ce5-6c024db50236".to_string()).await;

    // get_program_template().await;

    get_program_and_input_group("5793ec0c-d820-4613-bee9-46bf06dd6dbd".to_string()).await;


    // // // compress_folder("../risc_0_examples/basic_prime_test/methods", "./my_compressed_methods.tar").expect("Compression failed");
    // // // compress_folder("./folder_to_compress", "./my_compressed_methods.tar").expect("Compression failed");
    // // compress_folder("./methods", "./my_compressed_methods.tar").expect("Compression failed");

    // compress_folder_contents("./methods_test", "./my_compressed_methods.tar").expect("Compression failed");
    

    // // decompress_tar("./my_compressed_methods.tar", "./src/runner/methods").expect("Decompression failed")
    // decompress_tar("./my_compressed_methods.tar", "./src/runner/methods").expect("Decompression failed")
    // // decompress_tar("./downloaded_file.tar", "./my_decompressed_src").expect("Decompression failed")

    // get_organizations(None, None).await;

    // println!("");
    // println!("");
    // println!("");

    // get_organization_programs("210c3559-86d1-4bbb-999b-dcc1d27867ea".to_string(), None, None).await;

    // println!("");
    // println!("");
    // println!("");


    // get_general_programs(None, None).await;

    // // run_commands_loop();

}