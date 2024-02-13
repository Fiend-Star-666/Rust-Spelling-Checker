use crate::spell_check::spell_checker::SpellChecker;
use std::collections::{HashMap, HashSet};
use std::ffi::{c_char, c_int};
use std::sync::Mutex;
use rayon::prelude::*;
use rustacuda::memory::{CopyDestination, DeviceBuffer};
use std::ffi::CString;

extern "C" {
    fn suggest_corrections_kernel(
        unknown_words: *const *const c_char,
        corrections: *mut *mut c_char,
        num_words: c_int,
    );
}

pub struct WagnerFischerChecker {
    dictionary: HashSet<String>,
    cache: Mutex<HashMap<(String, String), usize>>,
}

impl WagnerFischerChecker {
    pub fn new(dictionary: HashSet<String>) -> Self {
        WagnerFischerChecker {
            dictionary,
            cache: Mutex::new(HashMap::new()),
        }
    }


    fn wagner_fischer(&self, s1: &str, s2: &str) -> usize {
        let mut cache = self.cache.lock().unwrap();

        // If the result is in the cache, return it
        if let Some(&result) = cache.get(&(s1.to_string(), s2.to_string())) {
            return result;
        }

        let mut matrix = vec![vec![0; s2.len() + 1]; s1.len() + 1];

        for i in 1..=s1.len() {
            matrix[i][0] = i;
        }

        for j in 1..=s2.len() {
            matrix[0][j] = j;
        }

        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                matrix[i + 1][j + 1] = *[
                    matrix[i][j + 1] + 1,
                    matrix[i + 1][j] + 1,
                    matrix[i][j] + cost,
                ]
                    .iter()
                    .min()
                    .unwrap();
            }
        }

        let result = matrix[s1.len()][s2.len()];

        // Store the result in the cache before returning it
        cache.insert((s1.to_string(), s2.to_string()), result);

        result
    }
}

impl SpellChecker for WagnerFischerChecker {
    fn check_word(&self, word: &str) -> bool {
        self.dictionary.contains(word)
    }

    fn suggest_correction(&self, word: &str) -> Vec<String> {
        let mut suggestions: Vec<_> = self.dictionary.par_iter()
            .map(|dict_word| (dict_word, self.wagner_fischer(word, dict_word)))
            .filter(|&(_, dist)| dist <= 2)  // You can adjust the threshold
            .collect();

        // Convert the suggestions to CStrings
        let c_suggestions: Vec<CString> = suggestions.iter()
            .map(|(word, _)| CString::new(word.as_str()).unwrap())
            .collect();

        // Convert the CStrings to raw pointers
        let p_suggestions: Vec<*const c_char> = c_suggestions.iter()
            .map(|c_string| c_string.as_ptr())
            .collect();

        // Convert the suggestions to Vec<i8>
        let i_suggestions: Vec<Vec<i8>> = suggestions.iter()
            .map(|(word, _)| word.as_bytes().iter().map(|&b| b as i8).collect())
            .collect();

        // Create a DeviceBuffer from each Vec<i8>
        let mut device_suggestions: Vec<DeviceBuffer<i8>> = i_suggestions.iter()
            .map(|i_suggestion| DeviceBuffer::from_slice(i_suggestion).unwrap())
            .collect();

        // Allocate memory on the GPU for the corrections
        let mut device_corrections = unsafe { DeviceBuffer::zeroed(suggestions.len()) }.unwrap();

        // Define the grid and block size for the CUDA kernel
        let grid_size = (suggestions.len() + 255) / 256;
        let block_size = 256;

        // Launch the CUDA kernel
        unsafe {
            suggest_corrections_kernel(
                device_suggestions.as_ptr() as *const *const c_char,
                device_corrections.as_device_ptr().as_raw() as *mut *mut c_char,
                suggestions.len() as c_int,
            );
        }

        // Copy the corrections from the GPU to the CPU
        let mut corrections = vec![0; suggestions.len()];
        device_corrections.copy_to(&mut corrections).unwrap();

        // Combine the words and their corrections into a vector of tuples
        let mut corrections: Vec<_> = suggestions.into_iter().zip(corrections.into_iter()).collect();

        // Sort the corrections by their distance
        corrections.sort_by_key(|&(_, dist)| dist);

        // Take the top 3 corrections
        corrections.into_iter()
            .take(3)
            .map(|((word, _), _)| word.clone())
            .collect::<Vec<String>>()
    }
}

//CPU

// fn suggest_correction(&self, word: &str) -> Vec<String> {
//     let mut suggestions: Vec<_> = self.dictionary.par_iter()
//         .map(|dict_word| (dict_word, self.wagner_fischer(word, dict_word)))
//         .filter(|&(_, dist)| dist <= 2)  // You can adjust the threshold
//         .collect();
//
//     // Sort the suggestions by their distance
//     suggestions.sort_by_key(|&(_, dist)| dist);
//
//     // Take the top 3 suggestions
//     suggestions.into_iter()
//         .take(3)
//         .map(|(word, _)| word.clone())
//         .collect()
// }

