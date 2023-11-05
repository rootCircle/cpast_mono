use std::path::Path;
use std::{fs, io};

pub fn read_file(file_path: &Path) -> io::Result<String> {
    let src_content = fs::read(file_path)?;
    Ok(String::from_utf8_lossy(&src_content)
        .parse::<String>()
        .expect("Error reading the file!"))
}

pub fn string_diff(source: &str, dest: &str) -> bool {
    source == dest
}