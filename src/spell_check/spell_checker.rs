pub trait SpellChecker {
    fn check_word(&self, word: &str) -> bool;
    fn suggest_correction(&self, word: &str) -> Vec<String>;
}
