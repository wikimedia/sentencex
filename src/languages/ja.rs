use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_lowercase_word_list;

#[derive(Debug, Clone)]
pub struct Japanese {}
static JAPANESE_ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_lowercase_word_list([include_str!("./abbrev/en.txt")]));

impl Language for Japanese {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &JAPANESE_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Japanese {}, "tests/ja.txt");
    }
}
