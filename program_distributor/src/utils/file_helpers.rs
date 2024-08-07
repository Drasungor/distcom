pub fn get_file_suffix(filename: &String) -> String {
    let filename_split: Vec<&str> = filename.split('.').collect(); // TODO: make the separation character a config attribute
    let file_suffix = filename_split[filename_split.len() - 1];
    file_suffix.to_string()
}

pub fn get_filename_without_suffix(filename: &String) -> String {
    let filename_split: Vec<&str> = filename.split('.').collect(); // TODO: make the separation character a config attribute
    let file_suffix = filename_split[filename_split.len() - 1];
    let file_suffix_index = filename.len() - file_suffix.len() - 1;
    filename[..file_suffix_index].to_string()
}