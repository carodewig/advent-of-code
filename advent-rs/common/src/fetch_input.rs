use curl::easy::Easy;
use std::{env::temp_dir, fs::File, io::Write, path::PathBuf};

const COOKIE: &str = "session=INSERT_SESSION_COOKIE_HERE";

fn url(year: u16, day: u8) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

fn local_path(year: u16, day: u8) -> PathBuf {
    let mut path = temp_dir();
    path.push(format!("{}_{}.txt", year, day));
    path
}

/// Download advent-of-code input file to a local path. Return that
/// path if the download succeeded.
fn download_to_local(year: u16, day: u8) -> Option<PathBuf> {
    let output_file_path = local_path(year, day);
    let mut output_file = File::create(output_file_path.clone()).ok()?;

    let mut task = Easy::new();
    task.url(&url(year, day)).ok()?;
    task.cookie(COOKIE).ok()?;
    task.write_function(move |data| {
        output_file.write_all(data).unwrap();
        Ok(data.len())
    })
    .ok()?;
    task.perform().ok()?;

    Some(output_file_path)
}

fn fetch_input_file_internal(year: u16, day: u8) -> (Option<PathBuf>, bool) {
    let output_file_path = local_path(year, day);
    if output_file_path.exists() {
        (Some(output_file_path), false)
    } else {
        (download_to_local(year, day), true)
    }
}

#[must_use]
pub fn fetch_input_file(year: u16, day: u8) -> Option<PathBuf> {
    fetch_input_file_internal(year, day).0
}

#[cfg(test)]
mod tests {
    use super::{download_to_local, fetch_input_file_internal};
    use std::fs::remove_file;

    #[test]
    fn successfully_downloads_file() {
        let file = download_to_local(2021, 6);
        assert!(file.is_some());
        remove_file(file.unwrap()).unwrap();
    }

    #[test]
    fn only_downloads_file_once() {
        let (file, ran_download) = fetch_input_file_internal(2021, 7);
        assert!(file.is_some());
        assert!(ran_download);

        let (file, ran_download) = fetch_input_file_internal(2021, 7);
        assert!(file.is_some());
        assert!(!ran_download);

        remove_file(file.unwrap()).unwrap();
    }
}
