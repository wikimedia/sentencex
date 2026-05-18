use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::language::continues_after_boundary;
use super::parse_lowercase_word_list;

#[derive(Debug, Clone)]
pub struct German {}

static GERMAN_ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    parse_lowercase_word_list([
        include_str!("./abbrev/de.txt"),
        include_str!("./abbrev/en.txt"),
    ])
});

const MONTHS: [&str; 12] = [
    "Januar",
    "Februar",
    "März",
    "April",
    "Mai",
    "Juni",
    "Juli",
    "August",
    "September",
    "Oktober",
    "November",
    "Dezember",
];

impl Language for German {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &GERMAN_ABBREVIATIONS
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        continues_after_boundary(text_after_boundary, &MONTHS)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(German {}, "tests/de.txt");
    }
}
