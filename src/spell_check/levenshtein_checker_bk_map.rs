
extern crate bk_tree;
use bk_tree::{BKTree, Metric};

// Define a wrapper function for levenshtein
fn levenshtein_metric(a: &str, b: &str) -> usize {
    levenshtein(a, b)
}

// Implement the Metric trait for your wrapper function
impl Metric<&str> for fn(&str, &str) -> usize {
    fn distance(&self, a: &str, b: &str) -> usize {
        self(a, b)
    }

    fn threshold_distance(&self, _threshold: usize) -> usize {
        // Return the threshold value or any logic you deem appropriate
        _threshold
    }
}

pub struct LevenshteinChecker {
    dictionary: BKTree<String, fn(&str, &str) -> usize>,
}

impl crate::spell_check::levenshtein_checker::LevenshteinChecker {
    pub fn new(words: Vec<String>) -> Self {
        let levenshtein_fn: fn(&str, &str) -> usize = levenshtein_metric;
        let mut tree = BKTree::new(levenshtein_fn);
        for word in words {
            tree.add(word);
        }
        crate::spell_check::levenshtein_checker::LevenshteinChecker { dictionary: tree }
    }
}



impl SpellChecker for crate::spell_check::levenshtein_checker::LevenshteinChecker {
    fn check_word(&self, word: &str) -> bool {
        // A word is correct if it's exactly in the dictionary (distance 0)
        !self.dictionary.find(word, 0).is_empty()
    }

    fn suggest_correction(&self, word: &str) -> Option<String> {
        self.dictionary.find(word, 2)  // Adjust the threshold as needed
            .into_iter()
            .min_by_key(|&(_, dist)| dist)
            .map(|(word, _)| word)  // No need to clone, as we're already getting the owned String
    }

}

