// pub fn tokenizer(text: &str) -> Vec<String> {
//     text.split_whitespace()
//         .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase())
//         .filter(|word| !word.is_empty())
//         .collect()
// }

// use unicode_segmentation::UnicodeSegmentation;
//
// pub fn tokenizer(text: &str) -> Vec<String> {
//     text.unicode_words() // Using unicode-segmentation for better word boundaries
//         .map(|word| word.to_lowercase()) // Convert each word to lowercase
//         .collect()
// }

// use regex::Regex;
//
// pub fn tokenizer(text: &str) -> Vec<String> {
//     let token_pattern = Regex::new(r"[a-zA-Z0-9]+('[a-z]+)?").unwrap();
//     token_pattern
//         .find_iter(text)
//         .map(|mat| mat.as_str().to_lowercase())
//         .collect()
// }

// use regex::Regex;
//
// pub fn tokenizer(text: &str) -> Vec<String> {
//     let token_pattern = Regex::new(r"[a-zA-Z0-9]+('[a-zA-Z0-9]+)?").unwrap();
//     token_pattern
//         .find_iter(text)
//         .map(|mat| handle_contractions(mat.as_str()).to_lowercase())
//         .collect()
// }
//
// fn handle_contractions(word: &str) -> String {
//     match word.to_lowercase().as_str() {
//         "ve" => "have",
//         "n't" => "not",
//         "'re" => "are",
//         "'ll" => "will",
//         "'m" => "am",
//         "'d" => "would",
//         "can't" => "cannot",
//         "won't" => "will not",
//         "don't" => "do not",
//         "doesn't" => "does not",
//         "didn't" => "did not",
//         "aren't" => "are not",
//         "isn't" => "is not",
//         "wasn't" => "was not",
//         "weren't" => "were not",
//         "haven't" => "have not",
//         "hasn't" => "has not",
//         "hadn't" => "had not",
//         "wouldn't" => "would not",
//         "shouldn't" => "should not",
//         "couldn't" => "could not",
//         "mightn't" => "might not",
//         "mustn't" => "must not",
//         "i'm" => "i am",
//         "you're" => "you are",
//         "he's" => "he is",
//         "she's" => "she is",
//         "it's" => "it is",
//         "we're" => "we are",
//         "they're" => "they are",
//         "i've" => "i have",
//         "you've" => "you have",
//         "we've" => "we have",
//         "they've" => "they have",
//         "i'd" => "i would",
//         "you'd" => "you would",
//         "he'd" => "he would",
//         "she'd" => "she would",
//         "we'd" => "we would",
//         "they'd" => "they would",
//         _ => word,
//     }.to_string()
// }

use regex::Regex;

pub fn tokenizer(text: &str) -> Vec<String> {
    let token_pattern = Regex::new(r"\b[\w']+\b").unwrap();
    token_pattern
        .find_iter(text)
        .map(|mat| mat.as_str().to_lowercase())
        .collect()
}

