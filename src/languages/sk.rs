use once_cell::sync::Lazy;

use crate::constants::ROMAN_NUMERALS;

use super::Language;

#[derive(Debug, Clone)]
pub struct Slovak {}
static SLOVAK_ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/sk.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

static SLOVAK_ALL_ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    let mut abbreviations = SLOVAK_ABBREVIATIONS.clone();
    abbreviations.extend(ROMAN_NUMERALS.iter().map(|&s| s.to_string()));
    abbreviations.extend(ROMAN_NUMERALS.iter().map(|&s| s.to_uppercase()));
    abbreviations
});

impl Language for Slovak {
    fn get_abbreviations(&self) -> &[String] {
        &SLOVAK_ALL_ABBREVIATIONS
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        use regex::Regex;
        use std::collections::HashSet;

        // Define the months as a HashSet for efficient lookups
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
        let months_set: HashSet<&str> = MONTHS.iter().cloned().collect();

        // Check if the text matches the regex pattern
        let regex = Regex::new(r"^\W*[0-9a-z]").unwrap();
        if regex.is_match(text_after_boundary) {
            return true;
        }

        // Extract the next word
        let next_word = text_after_boundary
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_matches(&['?', '!', '.'][..]);

        if next_word.is_empty() {
            return false;
        }

        // Check if the next word is in MONTHS or matches the capitalization rule
        let month_capitalized = next_word
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>();

        if months_set.contains(next_word) || months_set.contains(month_capitalized.as_str()) {
            return true;
        }

        false
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
