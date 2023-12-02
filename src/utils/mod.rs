use std::fs;

pub fn load_file(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

pub fn load_file_lines(path: &str) -> Vec<String> {
    load_file(path).lines().map( |s| { s.to_string() }).collect()
}