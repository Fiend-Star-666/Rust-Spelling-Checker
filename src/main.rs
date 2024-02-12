use std::collections::HashSet;
use crate::spell_check::spell_checker::SpellChecker;
use std::time::Instant;
use crate::spell_check::precomputed_levenshtein_checker::PrecomputedLevenshteinChecker;
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
    // pub mod levenshtein_checker_bk_map;
    pub mod soundex_checker;
}

pub fn main() {
    println!("Hello, world!");

    let dataset_file_path = "data/dataset/book.txt";

    // let dictionary_file_path = "data/dictionary/dict.txt";

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
    let unknown_words_set = unknown_words_levenshtein.iter().collect::<HashSet<&String>>();
    println!("set of unknown words: {:?}", unknown_words_set.len());

    //strip the set of numbers
    let unknown_words_set = unknown_words_set.iter()
        .filter(|word| !word.chars().any(|c| c.is_digit(10)))
        .cloned()
        .collect::<HashSet<&String>>();

    println!("set of unknown words: {:?}", unknown_words_set.len());

    let duration_leven_correction = Instant::now(); // Start timer

    let chunk_size = unknown_words_set.len() / rayon::current_num_threads();
    let unknown_words_vec: Vec<_> = unknown_words_set.clone().into_iter().collect();

    let fix_levenshtein_corrections: Vec<_> = unknown_words_vec.par_chunks(chunk_size)
        .map(|chunk| {
            chunk.iter()
                .map(|word| levenshtein_checker.suggest_correction(word.as_str()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let duration_leven_correction = duration_leven_correction.elapsed(); // End timer

    println!("set of unknown words: {:?}", unknown_words_set.len());
    println!("Time elapsed in checking unknown words using levenshtein correction: {:?}", duration_leven_correction);
    println!("Levenshtein corrections: {:?}", fix_levenshtein_corrections.len());
    println!("Levenshtein corrections: {:?}", fix_levenshtein_corrections);

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
}

