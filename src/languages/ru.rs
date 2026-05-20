use super::Language;
use super::parse_word_list;
use rustc_hash::FxHashSet;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct Russian {}

static PATTERN: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[0-9a-zа-я]").expect("Failed to compile regex"));

static ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_word_list([include_str!("./abbrev/ru.txt")]));

impl Language for Russian {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &ABBREVIATIONS
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        PATTERN.is_match(text_after_boundary)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Russian {}, "tests/ru.txt");
    }
}
