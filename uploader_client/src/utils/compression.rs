use tar::{Builder, Archive};
use std::fs::File;
use std::io;
use std::fs;


pub fn compress_folder(folder_path: &str, output_path: &str) -> io::Result<()> {
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

pub fn compress_folder_contents(folder_path: &str, output_path: &str) -> io::Result<()> {
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


pub fn decompress_tar(tar_path: &str, output_folder: &str) -> io::Result<()> {
    fs::create_dir_all(output_folder)?;
    let file = File::open(tar_path)?;
    let mut archive = Archive::new(file);
    archive.unpack(output_folder)?;
    Ok(())
}
