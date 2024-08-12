use std::fs;


pub fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn create_folder(path: &str) {
    if !folder_exists(path) {
        fs::create_dir_all(path).expect("Error in uploads folder creation")
    }
}
