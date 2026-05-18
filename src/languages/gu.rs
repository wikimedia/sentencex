use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_lowercase_word_list;

#[derive(Debug, Clone)]
pub struct Gujarati {}

static GUJARATI_ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    parse_lowercase_word_list([
        include_str!("./abbrev/gu.txt"),
        include_str!("./abbrev/en.txt"),
    ])
});
impl Language for Gujarati {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &GUJARATI_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Gujarati {}, "tests/gu.txt");
    }
}
