use std::fs::read_to_string;
use std::path::PathBuf;

#[must_use]
pub fn read_file_as_string(path: PathBuf) -> String {
    read_to_string(path)
        .expect("Unable to read file")
        .trim()
        .to_string()
}

#[must_use]
pub fn read_file_as_lines(path: PathBuf) -> Vec<String> {
    read_to_string(path)
        .expect("Unable to read file")
        .split('\n')
        .map(ToString::to_string)
        .collect()
}
