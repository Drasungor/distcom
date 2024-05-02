use std::env;
use std::fs::File;
use std::io::Write;
use std::io;
use tar::{Builder, Archive};
// use clap::{crate_name};
use clap::{crate_name, Parser, Subcommand};

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
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        /// a command
        #[clap(short = 'a', long = "keysa")]
        valuea: Option<String>,

        #[clap(short = 'b', long = "keysb")]
        valueb: Option<String>,
    },
    Set {
        key: String,
        value: String,
    }
}

#[tokio::main]
async fn main() {

    let response = reqwest::get("http://localhost:8080/program/b1eca5b7-5c28-459e-a64b-5244aabf1ab9").await.expect("Error in get");

    // Ensure the request was successful (status code 200)
    if response.status().is_success() {
        // Open a file to write the downloaded content
        let mut file = File::create("downloaded_file.tar").expect("Error in file creation");
        file.write_all(response.bytes().await.expect("Error in bytes get").as_ref()).expect("Errors in file write");

        
        println!("File downloaded successfully!");
    } else {
        println!("Failed to download file: {}", response.status());
    }

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
                    Commands::Get{valuea, valueb} => {
                        if (valuea.is_some()) {
                            println!("Get valuea: {}", valuea.unwrap());
                        }
                        if (valueb.is_some()) {
                            println!("Get valueb: {}", valueb.unwrap());
                        }
                    },
                    Commands::Set{key, value} => {
                        println!("Set key: {}, value: {}", key, value);
                    },
               }
            }
            Err(_) => {
                println!("That's not a valid command!");
            }
       };
    }

}