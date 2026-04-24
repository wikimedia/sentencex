use std::sync::LazyLock;

use regex::Regex;

use crate::constants::GLOBAL_SENTENCE_TERMINATORS;

use super::Language;

#[derive(Debug, Clone, Copy)]
pub struct Greek {}

static GREEK_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/el.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

static GREEK_SENTENCE_BREAK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!("[{};]+", GLOBAL_SENTENCE_TERMINATORS.join(""));
    Regex::new(&pattern).unwrap()
});

impl Language for Greek {
    fn get_abbreviations(&self) -> &[String] {
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
