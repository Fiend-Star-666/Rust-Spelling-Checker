use crate::spell_check::spell_checker::SpellChecker;
use strsim::levenshtein;
use std::collections::HashSet;
pub struct LevenshteinChecker {
    dictionary: HashSet<String>,
}

impl LevenshteinChecker {
    pub fn new(dictionary: HashSet<String>) -> Self {
        LevenshteinChecker { dictionary }
    }
}

impl SpellChecker for LevenshteinChecker {
    fn check_word(&self, word: &str) -> bool {
        self.dictionary.contains(word)
    }

    fn suggest_correction(&self, word: &str) -> Option<String> {
        self.dictionary.iter()
            .map(|dict_word| (dict_word, levenshtein(word, dict_word)))
            .filter(|&(_, dist)| dist <= 2)  // You can adjust the threshold
            .min_by_key(|&(_, dist)| dist)
            .map(|(word, _)| word.clone())
    }
}
