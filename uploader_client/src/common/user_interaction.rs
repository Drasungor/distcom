pub fn get_input_string() -> String {
    let mut buf = "".to_string();
    std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
    let line = buf.trim();
    line.to_string()
}

