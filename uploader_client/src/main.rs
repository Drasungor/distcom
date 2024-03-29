use std::env;
use std::fs::File;
use std::io;
use tar::{Builder, Archive};

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

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    println!("{}", args.len());

    // Check if the correct number of arguments is provided
    if args.len() != 3 {
        eprintln!("Usage: cargo run <folder_path> <output_path>");
        std::process::exit(1);
    }
    let folder_path = &args[1];
    let output_path = &args[2];

    // Call the function to compress the folder
    match compress_folder(folder_path, output_path) {
        Ok(_) => println!("Folder compressed successfully."),
        Err(err) => eprintln!("Error: {}", err),
    }
    match decompress_tar(output_path, &format!("./test/{}.", output_path)) {
    // match decompress_tar(output_path, output_path) {
        Ok(_) => println!("Folder decompressed successfully."),
        Err(err) => eprintln!("Error: {}", err),
    }
}