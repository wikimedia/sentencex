use regex::Regex;
use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_abbreviation_list;

#[derive(Debug, Clone)]
pub struct Danish {}
static DANISH_ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_abbreviation_list([include_str!("./abbrev/da.txt")]));

impl Language for Danish {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &DANISH_ABBREVIATIONS
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
        run_language_tests(Danish {}, "tests/da.txt");
    }
}
