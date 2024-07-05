use clap::{crate_name};

pub fn process_user_input() -> Vec<String> {
    let mut buf = format!("{} ", crate_name!());
    std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
    let line = buf.trim();
    let args = shlex::split(line).ok_or("error: Invalid quoting").unwrap();
    return args;
}
