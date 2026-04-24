use std::sync::LazyLock;

use regex::Regex;

use crate::constants::GLOBAL_SENTENCE_TERMINATORS;

use super::{English, Language};

#[derive(Debug, Clone, Copy)]
pub struct Burmese {}

static BURMESE_SENTENCE_BREAK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!("[{}၏]+", GLOBAL_SENTENCE_TERMINATORS.join(""));
    Regex::new(&pattern).unwrap()
});

impl Language for Burmese {
    fn get_abbreviations(&self) -> &[String] {
        English {}.get_abbreviations()
    }

    fn get_sentence_break_regex(&self) -> &'static Regex {
        &BURMESE_SENTENCE_BREAK_REGEX
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
