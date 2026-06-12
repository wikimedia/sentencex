use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use regex::Regex;

use crate::constants::GLOBAL_SENTENCE_TERMINATORS;

use super::Language;
use super::parse_lowercase_word_list;

#[derive(Debug, Clone)]
pub struct Greek {}

static GREEK_ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    parse_lowercase_word_list([
        include_str!("./abbrev/el.txt"),
        include_str!("./abbrev/en.txt"),
    ])
});

static GREEK_SENTENCE_BREAK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!(
        "[{};]+",
        GLOBAL_SENTENCE_TERMINATORS.iter().collect::<String>()
    );
    Regex::new(&pattern).unwrap()
});

impl Language for Greek {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &GREEK_ABBREVIATIONS
    }

    fn get_sentence_break_regex(&self) -> &'static Regex {
        &GREEK_SENTENCE_BREAK_REGEX
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
