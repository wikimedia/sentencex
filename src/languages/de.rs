use once_cell::sync::Lazy;

use super::Language;

#[derive(Debug, Clone)]
pub struct Deutch {}
static DEUTCH_ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/de.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for Deutch {
    fn get_abbreviations(&self) -> &[String] {
        &DEUTCH_ABBREVIATIONS
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        use regex::Regex;
        use std::collections::HashSet;

        // Define the months as a HashSet for efficient lookups
        const MONTHS: [&str; 12] = [
            "Januar",
            "Februar",
            "März",
            "April",
            "Mai",
            "Juni",
            "Juli",
            "August",
            "September",
            "Oktober",
            "November",
            "Dezember",
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
        run_language_tests(Deutch {}, "tests/de.txt");
    }
}
