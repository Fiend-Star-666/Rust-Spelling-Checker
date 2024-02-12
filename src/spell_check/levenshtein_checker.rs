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

    fn suggest_correction(&self, word: &str) -> Vec<String> {
        let mut suggestions = self.dictionary.iter()
            .map(|dict_word| (dict_word, levenshtein(word, dict_word)))
            .filter(|&(_, dist)| dist <= 2)  // You can adjust the threshold
            .collect::<Vec<(&String, usize)>>();

        // Sort the suggestions by their distance
        suggestions.sort_by_key(|&(_, dist)| dist);

        // Take the top 3 suggestions
        suggestions.into_iter()
            .take(3)
            .map(|(word, _)| word.clone())
            .collect()
    }
}
