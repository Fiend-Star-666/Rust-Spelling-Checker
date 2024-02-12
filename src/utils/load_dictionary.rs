use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn load_dictionary(file_path: &str) -> io::Result<HashSet<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let dictionary = reader.lines()
        .filter_map(Result::ok)
        .map(|word| word.to_lowercase())
        .collect::<HashSet<String>>();

    Ok(dictionary)
}
