use std::sync::LazyLock;

use crate::constants::ROMAN_NUMERALS;

use super::language::continues_after_boundary;
use super::Language;

#[derive(Debug, Clone)]
pub struct Slovak {}

static SLOVAK_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/sk.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

static SLOVAK_ALL_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut abbreviations = SLOVAK_ABBREVIATIONS.clone();
    abbreviations.extend(ROMAN_NUMERALS.iter().map(|&s| s.to_string()));
    abbreviations.extend(ROMAN_NUMERALS.iter().map(|&s| s.to_uppercase()));
    abbreviations
});

const MONTHS: [&str; 24] = [
    "Január",
    "Február",
    "Marec",
    "Apríl",
    "Máj",
    "Jún",
    "Júl",
    "August",
    "September",
    "Október",
    "November",
    "December",
    "Januára",
    "Februára",
    "Marca",
    "Apríla",
    "Mája",
    "Júna",
    "Júla",
    "Augusta",
    "Septembra",
    "Októbra",
    "Novembra",
    "Decembra",
];

impl Language for Slovak {
    fn get_abbreviations(&self) -> &[String] {
        &SLOVAK_ALL_ABBREVIATIONS
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
        run_language_tests(Slovak {}, "tests/sk.txt");
    }
}
