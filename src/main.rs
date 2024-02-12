use std::collections::HashSet;
use crate::spell_check::spell_checker::SpellChecker;
use std::time::Instant;

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

    // pub mod levenshtein_checker_bk_map;

    pub mod soundex_checker;
}

pub fn main() {
    println!("Hello, world!");

    let dataset_file_path = "data/dataset/book.txt";

    //let dictionary_file_path = "data/dictionary/dict.txt";

    let dictionary_file_path = "data/dictionary/insane-dict.txt";

    let dictionary = utils::load_dictionary::load_dictionary(dictionary_file_path).unwrap();
    let dataset = utils::read_dataset::read_dataset(dataset_file_path).unwrap();

    let dataset_words = utils::tokenizer::tokenizer(&dataset);
    let dictionary_words = dictionary.iter().collect::<Vec<&String>>();

    //use hashmap lookup
    let hashmap_lookup = spell_check::hash_map_look_up::HashMapLookup::new(dictionary.clone());


    let start_hash_look_up = Instant::now(); // Start timer

    let unknown_words_hashmap = dataset_words.iter()
        .filter(|word| !hashmap_lookup.check_word(word.as_str()))
        .cloned()
        .collect::<Vec<String>>();

    let duration_hash_look_up = start_hash_look_up.elapsed(); // End timer

    let levenshtein_checker = spell_check::levenshtein_checker::LevenshteinChecker::new(dictionary.clone().into_iter().collect());
    let levenshtein_look_up = Instant::now(); // Start timer

    let unknown_words_levenshtein = dataset_words.iter()
        .filter(|word| !levenshtein_checker.check_word(word.as_str()))
        .cloned()
        .collect::<Vec<String>>();

    let duration_levenshtein_look_up = levenshtein_look_up.elapsed(); // End timer

    println!("Unknown words hash: {:?}", unknown_words_hashmap.len());
    println!("Unknown words leven: {:?}", unknown_words_levenshtein.len());
    println!("Dictionary words: {}", dictionary_words.len());
    println!("Dataset words: {}", dataset_words.len());
    println!("Time elapsed in checking unknown words using hashMap: {:?}", duration_hash_look_up);
    println!("Time elapsed in checking unknown words using levenshtein: {:?}", duration_levenshtein_look_up);



    //create a set of unknown words
    let unknown_words_set = unknown_words_levenshtein.iter().collect::<std::collections::HashSet<&String>>();
    println!("set of unknown words: {:?}", unknown_words_set.len());

    //strip the set of numbers
    let unknown_words_set = unknown_words_set.iter()
        .filter(|word| !word.chars().any(|c| c.is_digit(10)))
        .cloned()
        .collect::<std::collections::HashSet<&String>>();

    println!("set of unknown words: {:?}", unknown_words_set.len());

    let duration_leven_correction = Instant::now(); // Start timer

    let fix_levenshtein_corrections = unknown_words_set.par_iter()
        .map(|word| {
            let correction = levenshtein_checker.suggest_correction(word.as_str());
            correction
        })
        .collect::<Vec<Option<String>>>();

    let duration_leven_correction = duration_leven_correction.elapsed(); // End timer

    println!("set of unknown words: {:?}", unknown_words_set.len());
    println!("Time elapsed in checking unknown words using levenshtein correction: {:?}", duration_leven_correction);
    println!("Levenshtein corrections: {:?}", fix_levenshtein_corrections.len());
    println!("Levenshtein corrections: {:?}", fix_levenshtein_corrections);

}

