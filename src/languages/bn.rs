use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_lowercase_word_list;

#[derive(Debug, Clone)]
pub struct Bengali {}
static BENGALI_ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    parse_lowercase_word_list([
        include_str!("./abbrev/bn.txt"),
        include_str!("./abbrev/en.txt"),
    ])
});

impl Language for Bengali {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &BENGALI_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Bengali {}, "tests/bn.txt");
    }
}
