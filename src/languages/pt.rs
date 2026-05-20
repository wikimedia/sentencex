use crate::constants::ROMAN_NUMERALS;

use super::Language;

use rustc_hash::FxHashSet;
use std::sync::LazyLock;

static ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    let mut abbreviations: FxHashSet<String> = include_str!("./abbrev/pt.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect();

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
