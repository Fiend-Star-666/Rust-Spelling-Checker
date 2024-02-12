use std::collections::HashSet;
use crate::spell_check::spell_checker::SpellChecker;
use std::time::Instant;
use crate::spell_check::precomputed_levenshtein_checker::PrecomputedLevenshteinChecker;
use crate::spell_check::wagner_fischer::WagnerFischerChecker;

extern crate rayon;

use rayon::prelude::*;

mod utils {
    pub mod io;
    pub mod tokenizer;
    pub mod read_dataset;
    pub mod load_dictionary;
}

mod cuda {
    // pub mod levenshtein_corrections_kernel;
}

mod spell_check {
    pub mod spell_checker;
    pub mod hash_map_look_up;
    pub mod levenshtein_checker;
    pub mod precomputed_levenshtein_checker;
    pub mod wagner_fischer;
    pub mod soundex_checker;
    // pub mod levenshtein_checker_bk_map;

}

pub fn main() {
    let dataset_file_path = "data/dataset/book.txt";

    // let dictionary_file_path = "data/dictionary/dict.txt";

    let dictionary_file_path = "data/dictionary/insane-dict.txt";

    let (dictionary, dataset) = load_data(dictionary_file_path, dataset_file_path);
    let (dataset_words, dictionary_words) = tokenize_data(&dataset, &dictionary);

    let hashmap_lookup = spell_check::hash_map_look_up::HashMapLookup::new(dictionary.clone());
    let (unknown_words_hashmap, duration_hash_look_up) = check_unknown_words(&dataset_words, &hashmap_lookup);

    let levenshtein_checker = spell_check::levenshtein_checker::LevenshteinChecker::new(dictionary.clone().into_iter().collect());
    let (unknown_words_levenshtein, duration_levenshtein_look_up) = check_unknown_words(&dataset_words, &levenshtein_checker);

    print_unknown_words_info(&unknown_words_hashmap, &unknown_words_levenshtein, &dictionary_words, &dataset_words, duration_hash_look_up, duration_levenshtein_look_up);

    let unknown_words_set = filter_unknown_words(&unknown_words_levenshtein);

    let chunk_size = unknown_words_set.len() / rayon::current_num_threads();
    let (fix_levenshtein_corrections, duration_leven_correction) = suggest_corrections(&unknown_words_set, &levenshtein_checker, chunk_size);

    print_correction_info(&unknown_words_set, duration_leven_correction, &fix_levenshtein_corrections);

    let wagner_fischer_checker = WagnerFischerChecker::new(dictionary.clone());
    let (unknown_words_wagner_fischer, duration_wagner_fischer_look_up) = check_unknown_words(&dataset_words, &wagner_fischer_checker);

    print_unknown_words_info(&unknown_words_hashmap, &unknown_words_wagner_fischer, &dictionary_words, &dataset_words, duration_hash_look_up, duration_wagner_fischer_look_up);

    let unknown_words_set = filter_unknown_words(&unknown_words_wagner_fischer);

    let chunk_size = unknown_words_set.len() / rayon::current_num_threads();
    let (fix_wagner_fischer_corrections, duration_wagner_fischer_correction) = suggest_corrections(&unknown_words_set, &wagner_fischer_checker, chunk_size);

    print_correction_info(&unknown_words_set, duration_wagner_fischer_correction, &fix_wagner_fischer_corrections);

}

fn load_data(dictionary_file_path: &str, dataset_file_path: &str) -> (HashSet<String>, String) {
    let dictionary = utils::load_dictionary::load_dictionary(dictionary_file_path).unwrap();
    let dataset = utils::read_dataset::read_dataset(dataset_file_path).unwrap();
    (dictionary, dataset)
}

fn tokenize_data<'a>(dataset: &str, dictionary: &'a HashSet<String>) -> (Vec<String>, Vec<&'a String>) {
    let dataset_words = utils::tokenizer::tokenizer(dataset);
    let dictionary_words = dictionary.iter().collect::<Vec<&'a String>>();
    (dataset_words, dictionary_words)
}

fn check_unknown_words(dataset_words: &Vec<String>, checker: &dyn SpellChecker) -> (Vec<String>, std::time::Duration) {
    let start = Instant::now();
    let unknown_words = dataset_words.iter()
        .filter(|word| !checker.check_word(word.as_str()))
        .cloned()
        .collect::<Vec<String>>();
    let duration = start.elapsed();
    (unknown_words, duration)
}

fn filter_unknown_words(unknown_words: &Vec<String>) -> HashSet<&String> {
    let unknown_words_set = unknown_words.iter().collect::<HashSet<&String>>();
    unknown_words_set.iter()
        .filter(|word| !word.chars().any(|c| c.is_digit(10)))
        .cloned()
        .collect::<HashSet<&String>>()
}

fn suggest_corrections(unknown_words_set: &HashSet<&String>, checker: &dyn SpellChecker, chunk_size: usize) -> (Vec<Vec<String>>, std::time::Duration) {
    let start = Instant::now();
    let unknown_words_vec: Vec<_> = unknown_words_set.clone().into_iter().collect();
    let corrections: Vec<_> = unknown_words_vec.par_chunks(chunk_size)
        .map(|chunk| {
            chunk.iter()
                .map(|word| checker.suggest_correction(word.as_str()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let duration = start.elapsed();
    (corrections, duration)
}

fn print_unknown_words_info(unknown_words_hashmap: &Vec<String>, unknown_words_levenshtein: &Vec<String>, dictionary_words: &Vec<&String>, dataset_words: &Vec<String>, duration_hash_look_up: std::time::Duration, duration_levenshtein_look_up: std::time::Duration) {
    println!("Unknown words hash: {:?}", unknown_words_hashmap.len());
    println!("Unknown words leven: {:?}", unknown_words_levenshtein.len());
    println!("Dictionary words: {}", dictionary_words.len());
    println!("Dataset words: {}", dataset_words.len());
    println!("Time elapsed in checking unknown words using hashMap: {:?}", duration_hash_look_up);
    println!("Time elapsed in checking unknown words using levenshtein: {:?}", duration_levenshtein_look_up);
}

fn print_correction_info(unknown_words_set: &HashSet<&String>, duration_leven_correction: std::time::Duration, fix_levenshtein_corrections: &Vec<Vec<String>>) {
    println!("set of unknown words: {:?}", unknown_words_set.len());
    println!("Time elapsed in checking unknown words using levenshtein correction: {:?}", duration_leven_correction);
    println!("Levenshtein corrections: {:?}", fix_levenshtein_corrections.len());
    println!("Levenshtein corrections: {:?}", fix_levenshtein_corrections);
}


// let duration_precomputed_levenshtein = Instant::now(); // Start timer
//
// let dictionary_vec: Vec<_> = dictionary.into_iter().collect();
// let precomputed_levenshtein_checker = PrecomputedLevenshteinChecker::new(dictionary_vec.clone());
//
// let unknown_words_vec: Vec<_> = unknown_words_set.into_iter().collect();
//
// let fix_precomputed_levenshtein_corrections: Vec<_> = unknown_words_vec.par_chunks(chunk_size)
//     .map(|chunk| {
//         chunk.iter()
//             .map(|word| precomputed_levenshtein_checker.suggest_correction(word.as_str()))
//             .collect::<Vec<_>>()
//     })
//     .flatten()
//     .collect();
//
// let duration_precomputed_levenshtein_completed = duration_precomputed_levenshtein.elapsed(); // End timer
//
// println!("Time elapsed in checking unknown words using precomputed levenshtein: {:?}", duration_precomputed_levenshtein_completed);
// println!("Precomputed Levenshtein corrections: {:?}", fix_precomputed_levenshtein_corrections.len());
// println!("Precomputed Levenshtein corrections: {:?}", fix_precomputed_levenshtein_corrections);

