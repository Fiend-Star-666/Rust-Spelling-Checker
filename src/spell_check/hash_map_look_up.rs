use std::collections::HashSet;
use crate::spell_check::spell_checker::SpellChecker;

pub struct HashMapLookup {
    dictionary: HashSet<String>,
}

impl HashMapLookup {
    pub fn new(dictionary: HashSet<String>) -> Self {
        HashMapLookup { dictionary }
    }
}

impl SpellChecker for HashMapLookup {
    fn check_word(&self, word: &str) -> bool {
        self.dictionary.contains(word)
    }

    fn suggest_correction(&self, _word: &str) -> Option<String> {
        // Suggestion logic can be implemented as needed
        None
    }
}



// let unknown_words = dataset_words.iter()
// .filter(|word| !dictionary.contains(word.as_str()))
// .cloned()
// .collect::<Vec<String>>();
