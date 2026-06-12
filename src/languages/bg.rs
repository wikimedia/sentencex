use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_lowercase_word_list;

#[derive(Debug, Clone)]
pub struct Bulgarian {}
static BULGARIAN_ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_lowercase_word_list([include_str!("./abbrev/bg.txt")]));

impl Language for Bulgarian {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &BULGARIAN_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Bulgarian {}, "tests/bg.txt");
    }
}
