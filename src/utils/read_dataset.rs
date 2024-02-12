use std::fs;

pub fn read_dataset(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}
