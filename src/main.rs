
use rustacuda::prelude::*;
use std::error::Error;
use std::ffi::CString;
use std::collections::HashSet;
use crate::spell_check::spell_checker::SpellChecker;
use std::time::Instant;
use crate::spell_check::precomputed_levenshtein_checker::PrecomputedLevenshteinChecker;
use crate::spell_check::wagner_fischer::WagnerFischerChecker;
use log::{info, debug};
use rustacuda::prelude::*;

extern crate rayon;

use rayon::prelude::*;

mod utils {
    pub mod io;
    pub mod tokenizer;
    pub mod read_dataset;
    pub mod load_dictionary;
}

mod cuda {
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

pub fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Initialize the CUDA API
    rustacuda::init(CudaFlags::empty()).expect("Could not initialize CUDA");

    // Get the first device
    let device = Device::get_device(0).expect("Could not get device");

    // Create a context associated with this device
    let _context = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)
        .expect("Could not create CUDA context");

    // Load the CUDA module
    let ptx_content = include_str!("../src/cuda/suggest_corrections_kernel.ptx");
    let ptx = CString::new(ptx_content)?;
    let module = Module::load_from_string(&ptx)?;

    let normal_dictionary_file_path = "data/dictionary/dict.txt";
    let dataset_file_path = "data/dataset/book.txt";
    let dictionary_file_path = "data/dictionary/insane-dict.txt";

    let (dictionary, dataset) = load_data(dictionary_file_path, dataset_file_path);
    let (dataset_words, dictionary_words) = tokenize_data(&dataset, &dictionary);

    let hashmap_lookup = spell_check::hash_map_look_up::HashMapLookup::new(dictionary.clone());
    let levenshtein_checker = spell_check::levenshtein_checker::LevenshteinChecker::new(dictionary.clone().into_iter().collect());
    let wagner_fischer_checker = WagnerFischerChecker::new(dictionary.clone());

    let checkers: Vec<(&dyn SpellChecker, &str)> = vec![
        (&hashmap_lookup, "hashmap"),
        (&levenshtein_checker, "levenshtein"),
        (&wagner_fischer_checker, "wagner_fischer"),
    ];

    for (checker, name) in checkers {
        let (unknown_words, duration_look_up) = check_unknown_words(&dataset_words, checker);
        print_unknown_words_info(&unknown_words, &dictionary_words, &dataset_words, duration_look_up, name);

        let unknown_words_set = filter_unknown_words(&unknown_words);
        let chunk_size = unknown_words_set.len() / rayon::current_num_threads();
        let (corrections, duration_correction) = suggest_corrections(&unknown_words_set, checker, chunk_size);

        print_correction_info(&unknown_words_set, duration_correction, &corrections, name);
    }
    Ok(())
}

fn load_data(dictionary_file_path: &str, dataset_file_path: &str) -> (HashSet<String>, String) {
    let dictionary = utils::load_dictionary::load_dictionary(dictionary_file_path).unwrap();
    let dataset = utils::read_dataset::read_dataset(dataset_file_path).unwrap();
    (dictionary, dataset)
}

fn tokenize_data<'a>(dataset: &str, dictionary: &'a HashSet<String>) -> (Vec<String>, Vec<&'a String>) {
    let dataset_words = utils::tokenizer::tokenizer(dataset);
    let dictionary_words = dictionary.par_iter().collect::<Vec<&'a String>>();
    (dataset_words, dictionary_words)
}

fn check_unknown_words(dataset_words: &Vec<String>, checker: &dyn SpellChecker) -> (HashSet<String>, std::time::Duration) {
    let start = Instant::now();
    let unknown_words = dataset_words.par_iter()
        .filter(|word| !checker.check_word(word.as_str()))
        .cloned()
        .collect::<HashSet<String>>();
    let duration = start.elapsed();
    (unknown_words, duration)
}

fn filter_unknown_words(unknown_words: &HashSet<String>) -> HashSet<&String> {
    unknown_words.iter()
        .filter(|word| !word.chars().any(|c| c.is_digit(10)))
        .collect::<HashSet<&String>>()
}

fn suggest_corrections(unknown_words_set: &HashSet<&String>, checker: &dyn SpellChecker, chunk_size: usize) -> (Vec<Vec<String>>, std::time::Duration) {
    let start = Instant::now();
    let unknown_words_vec: Vec<_> = unknown_words_set.clone().into_iter().collect();
    let corrections: Vec<_> = unknown_words_vec.par_chunks(chunk_size)
        .map(|chunk| {
            chunk.par_iter()
                .map(|word| checker.suggest_correction(word.as_str()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let duration = start.elapsed();
    (corrections, duration)
}

fn print_unknown_words_info(unknown_words: &HashSet<String>, dictionary_words: &Vec<&String>, dataset_words: &Vec<String>, duration: std::time::Duration, name: &str) {
    info!("__________________________________________________________________________");
    info!("Unknown words {}: {:?}", name, unknown_words.len());
    info!("Dictionary words: {}", dictionary_words.len());
    info!("Dataset words: {}", dataset_words.len());
    info!("Time elapsed in checking unknown words using {}: {:?}", name, duration);
    info!("__________________________________________________________________________");
}

fn print_correction_info(unknown_words_set: &HashSet<&String>, duration: std::time::Duration, corrections: &Vec<Vec<String>>, name: &str) {
    info!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    info!("set of unknown words: {:?}", unknown_words_set.len());
    info!("Time elapsed in checking unknown words using {} correction: {:?}", name, duration);

    let non_empty_corrections: Vec<_> = corrections.iter().filter(|c| !c.is_empty()).collect();

    info!("{} corrections: {:?}", name, non_empty_corrections.len());
    debug!("{} corrections: {:?}", name, non_empty_corrections);
    info!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
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

// fn suggest_corrections_cuda(unknown_words_set: &HashSet<&String>, checker: &dyn SpellChecker, chunk_size: usize) -> (Vec<Vec<String>>, std::time::Duration) {
//     let start = Instant::now();
//
//     // Convert the unknown_words_set to a Vec and allocate memory on the GPU
//     let unknown_words_vec: Vec<_> = unknown_words_set.clone().into_iter().collect();
//     let mut device_unknown_words = DeviceBuffer::from_slice(&unknown_words_vec).unwrap();
//
//     // Allocate memory on the GPU for the corrections
//     let mut device_corrections = DeviceBuffer::zeros(unknown_words_vec.len()).unwrap();
//
//     // Define the grid and block size for the CUDA kernel
//     let grid_size = (unknown_words_vec.len() + 255) / 256;
//     let block_size = 256;
//
//     // Launch the CUDA kernel
//     unsafe {
//         suggest_corrections_kernel<<<grid_size, block_size>>>(
//             device_unknown_words.as_device_ptr(),
//             device_corrections.as_device_ptr(),
//             unknown_words_vec.len()
//         );
//     }
//
//     // Copy the corrections from the GPU to the CPU
//     let mut corrections = vec![0; unknown_words_vec.len()];
//     device_corrections.copy_to(&mut corrections).unwrap();
//
//     let duration = start.elapsed();
//     (corrections, duration)
// }