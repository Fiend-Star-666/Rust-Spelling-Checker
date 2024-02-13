use std::collections::HashMap;
use strsim::levenshtein;

pub struct PrecomputedLevenshteinChecker {
    dictionary: Vec<String>,
    distances: HashMap<(String, String), usize>,
}

impl PrecomputedLevenshteinChecker {
    fn check_word(&self, word: &str) -> bool {
        self.dictionary.contains(&word.to_string())
    }

    pub fn new(dictionary: Vec<String>) -> Self {
        let mut distances = HashMap::new();

        for (i, word1) in dictionary.iter().enumerate() {
            for word2 in dictionary[i + 1..].iter() {
                let distance = levenshtein(word1, word2);
                distances.insert((word1.clone(), word2.clone()), distance);
                distances.insert((word2.clone(), word1.clone()), distance);
            }
        }

        PrecomputedLevenshteinChecker {
            dictionary,
            distances,
        }
    }

    pub fn suggest_correction(&self, word: &str) -> Vec<String> {
        let mut suggestions = self
            .dictionary
            .iter()
            .map(|dict_word| {
                (
                    dict_word,
                    *self
                        .distances
                        .get(&(word.to_string(), dict_word.clone()))
                        .unwrap_or(&usize::MAX),
                )
            })
            .filter(|&(_, dist)| dist <= 2) // You can adjust the threshold
            .collect::<Vec<(&String, usize)>>();

        // Sort the suggestions by their distance
        suggestions.sort_by_key(|&(_, dist)| dist);

        // Take the top 3 suggestions
        suggestions
            .into_iter()
            .take(3)
            .map(|(word, _)| word.clone())
            .collect()
    }
}
