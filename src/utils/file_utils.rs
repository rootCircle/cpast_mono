use std::path::Path;
use std::{fs, io};

#[allow(dead_code)]
pub fn read_file(file_path: &Path) -> io::Result<String> {
    let file_content = fs::read(file_path)?;
    return Ok(String::from_utf8_lossy(&file_content)
        .parse::<String>()
        .expect("Error reading the file!"));
}

pub fn string_diff(source: &str, dest: &str) -> bool {
    source == dest
}
