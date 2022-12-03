mod fetch_input;
mod read_file;
pub use fetch_input::fetch_input_file;
pub use read_file::{read_file_as_lines, read_file_as_string};

pub fn read_input_as_string(year: u16, day: u8) -> Option<String> {
    Some(read_file_as_string(fetch_input_file(year, day)?))
}
