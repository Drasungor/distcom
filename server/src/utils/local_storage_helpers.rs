use std::fs;

pub fn clear_directory(folder_path: &str) {
    // let downloads_folder_path = "./downloads";

    let entries = fs::read_dir(folder_path).expect("Failed reading the downloads folder");

    for entry in entries {
        let dir_entry = entry.expect("Error in entry parsing");
        let file_path = dir_entry.path();

        // Check if it's a file
        if file_path.is_file() {
            // Attempt to delete the file

            // TODO: 
            fs::remove_file(&file_path).expect("Error in file deletion");
            println!("Deleted file: {:?}", file_path);
        }
    }
}