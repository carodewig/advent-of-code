mod fetch_input;
mod location;
mod read_file;

pub use fetch_input::{delete_input_file, fetch_input_file};
pub use location::{Location, Step};
use read_file::{read_file_as_lines, read_file_as_string};

#[must_use]
pub fn read_input_as_string(year: u16, day: u8) -> Option<String> {
    read_file_as_string(fetch_input_file(year, day)?)
}

#[must_use]
pub fn read_input_as_lines(year: u16, day: u8) -> Option<Vec<String>> {
    read_file_as_lines(fetch_input_file(year, day)?)
}
