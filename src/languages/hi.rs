use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_word_list;

#[derive(Debug, Clone)]
pub struct Hindi {}
static HINDI_ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    parse_word_list([
        include_str!("./abbrev/hi.txt"),
        include_str!("./abbrev/en.txt"),
    ])
});

impl Language for Hindi {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &HINDI_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Hindi {}, "tests/hi.txt");
    }
}
