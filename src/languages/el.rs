use regex::Regex;

use crate::constants::GLOBAL_SENTENCE_TERMINATORS;

use super::Language;

#[derive(Debug, Clone)]
pub struct Greek {}

impl Language for Greek {
    fn get_abbreviations(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_sentence_break_regex(&self) -> Regex {
        let pattern = format!("[{};]+", GLOBAL_SENTENCE_TERMINATORS.join(""));
        Regex::new(&pattern).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Greek {}, "tests/el.txt");
    }
}
