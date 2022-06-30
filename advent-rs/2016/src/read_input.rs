use std::fs::read_to_string;
use std::path::PathBuf;

fn resource_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from("resources/");
    path.push(filename);
    path
}

pub fn read_file(filename: &str) -> String {
    let path = resource_path(filename);
    read_to_string(path)
        .expect("Unable to read file")
        .trim()
        .to_string()
}
