use regex::Regex;

use crate::constants::GLOBAL_SENTENCE_TERMINATORS;

use super::{English, Language};

#[derive(Debug, Clone)]
pub struct Burmese {}

impl Language for Burmese {
    fn get_abbreviations(&self) -> Vec<String> {
        English {}.get_abbreviations()
    }

    fn get_sentence_break_regex(&self) -> Regex {
        let pattern = format!("[{}၏]+", GLOBAL_SENTENCE_TERMINATORS.join(""));
        Regex::new(&pattern).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Burmese {}, "tests/my.txt");
    }
}
