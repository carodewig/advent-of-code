use std::fs::read_to_string;
use std::path::PathBuf;

#[must_use]
pub fn read_file_as_string(path: PathBuf) -> Option<String> {
    Some(read_to_string(path)
        .ok()?
        .trim()
        .to_string())
}

#[must_use]
pub fn read_file_as_lines(path: PathBuf) -> Option<Vec<String>> {
    Some(read_to_string(path)
        .ok()?
        .split('\n')
        .map(ToString::to_string)
        .collect())
}
