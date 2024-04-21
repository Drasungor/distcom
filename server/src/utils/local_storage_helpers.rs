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

            // If the file deletion fails it might be because the file is being used by the server, it can be deleted
            // in another moment
            let _ = fs::remove_file(&file_path);
        }
    }
}