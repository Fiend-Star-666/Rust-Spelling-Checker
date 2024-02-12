pub trait SpellChecker: Sync {
    fn check_word(&self, word: &str) -> bool;
    fn suggest_correction(&self, word: &str) -> Vec<String>;
}
