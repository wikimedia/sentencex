use crate::constants::ROMAN_NUMERALS;

use super::Language;
use super::parse_abbreviation_list;

use rustc_hash::FxHashSet;
use std::sync::LazyLock;

static ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    let mut abbreviations = parse_abbreviation_list([include_str!("./abbrev/pt.txt")]);
    abbreviations.extend(ROMAN_NUMERALS.iter().map(|&s| s.to_string()));
    abbreviations.extend(ROMAN_NUMERALS.iter().map(|&s| s.to_uppercase()));
    abbreviations
});

#[derive(Debug, Clone)]
pub struct Portuguese {}

impl Language for Portuguese {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Portuguese {}, "tests/pt.txt");
    }
}
