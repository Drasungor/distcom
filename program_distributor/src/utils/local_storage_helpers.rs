use std::fs;
use tar::Builder;
use std::fs::File;
use std::io;


pub fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn create_folder(path: &str) {
    if !folder_exists(path) {
        fs::create_dir(path).expect("Error in uploads folder creation")
    }
}

// pub fn clear_directory(folder_path: &str) {
//     // let downloads_folder_path = "./downloads";

//     let entries = fs::read_dir(folder_path).expect("Failed reading the downloads folder");

//     for entry in entries {
//         let dir_entry = entry.expect("Error in entry parsing");
//         let current_path = dir_entry.path();

//         // Check if it's a file
//         if current_path.is_file() {
//             // Attempt to delete the file

//             // If the file deletion fails it might be because the file is being used by the server, it can be deleted
//             // in another moment
//             let _ = fs::remove_file(&current_path);
//         } else if current_path.is_dir() {
//             fs::remove_dir_all(current_path);
//         }
//     }
// }

// pub fn clear_aux_directories() {
//     let downloads_folder = "./downloads";
//     create_folder(downloads_folder);
//     clear_directory(downloads_folder);

//     let aux_files_folder = "./aux_files";
//     create_folder(aux_files_folder);
//     clear_directory(aux_files_folder);
// }


pub fn compress_folder_contents(folder_path: &str, output_path: &str) -> io::Result<()> {
    let file = File::create(output_path)?;
    let mut builder = Builder::new(file);
    let folder_contents = fs::read_dir(folder_path).expect("Error in ");

    for entry in folder_contents {
        let unwrapped_entry = entry.expect("Error in folder entry processing");
        let path = unwrapped_entry.path();

        let entry_name = unwrapped_entry.file_name().into_string().expect("Error in converion from OsString to string");
        let entry_path = format!("{}/{}", folder_path, entry_name);

        if path.is_dir() {
            builder.append_dir_all(format!("./{}", entry_name), entry_path).expect("Error in directory appending");
        } else {
            builder.append_path_with_name(path, entry_name).expect("Error in directory appending");
        }

    }
    builder.finish()?;
    Ok(())
}