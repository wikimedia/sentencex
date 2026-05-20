use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_word_list;

#[derive(Debug, Clone)]
pub struct Dutch {}

static DUTCH_ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_word_list([include_str!("./abbrev/nl.txt")]));
impl Language for Dutch {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &DUTCH_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Dutch {}, "tests/nl.txt");
    }
}
