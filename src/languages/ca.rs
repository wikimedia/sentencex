use regex::Regex;
use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_lowercase_word_list;

#[derive(Debug, Clone)]
pub struct Catalan {}
static CATALAN_ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_lowercase_word_list([include_str!("./abbrev/es.txt")]));

impl Language for Catalan {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &CATALAN_ABBREVIATIONS
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        let regex = Regex::new(r"^\W*[0-9a-z]").unwrap();
        regex.is_match(text_after_boundary)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Catalan {}, "tests/ca.txt");
    }
}
