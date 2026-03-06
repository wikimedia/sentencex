use std::sync::LazyLock;

use regex::Regex;

use crate::constants::GLOBAL_SENTENCE_TERMINATORS;

use super::{English, Language};

#[derive(Debug, Clone)]
pub struct Armenian {}

static ARMENIAN_SENTENCE_BREAK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let hy_terminators: Vec<&str> = GLOBAL_SENTENCE_TERMINATORS
        .iter()
        .filter(|&&c| c != ".")
        .cloned()
        .collect();
    let pattern = format!("[{};։՜:]+", hy_terminators.join(""));
    Regex::new(&pattern).unwrap()
});

impl Language for Armenian {
    fn get_abbreviations(&self) -> &[String] {
        English {}.get_abbreviations()
    }

    fn get_sentence_break_regex(&self) -> &'static Regex {
        &ARMENIAN_SENTENCE_BREAK_REGEX
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Armenian {}, "tests/hy.txt");
    }
}
