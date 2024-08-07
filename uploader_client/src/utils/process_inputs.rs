use clap::crate_name;

use crate::common;

pub fn process_user_input() -> Vec<String> {
    let mut buf = format!("{} ", crate_name!());
    std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
    let line = buf.trim();
    
    shlex::split(line).ok_or("error: Invalid quoting").unwrap()
}

pub fn process_page_size(limit: Option<usize>) -> usize {
    if let Some(limit_value) = limit {
        return limit_value;
    }
    common::config::CONFIG_OBJECT.max_page_size
}

pub fn process_previously_set_page_size(previous_limit: usize, limit: Option<usize>) -> usize {
    if let Some(limit_value) = limit {
        return limit_value;
    }
    previous_limit
}