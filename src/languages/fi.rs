use once_cell::sync::Lazy;

use super::Language;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Finnish {}

static FINNISH_ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/fi.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for Finnish {
    fn get_abbreviations(&self) -> &[String] {
        &FINNISH_ABBREVIATIONS
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        // Check if the text matches the regex pattern
        let regex = Regex::new(r"^\W*[0-9a-z]").unwrap();
        if regex.is_match(text_after_boundary) {
            return true;
        }

        // Extract the next word
        let next_word = text_after_boundary.split_whitespace().next().unwrap_or("");

        // Check conditions for the next word
        if next_word.is_empty() {
            return false;
        }

        let months = [
            "tammikuu",
            "helmikuu",
            "maaliskuu",
            "huhtikuu",
            "toukokuu",
            "kesäkuu",
            "heinäkuu",
            "elokuu",
            "syyskuu",
            "lokakuu",
            "marraskuu",
            "joulukuu",
        ];

        if months.contains(&next_word)
            || months.contains(
                &format!(
                    "{}{}",
                    next_word.chars().next().unwrap_or_default().to_uppercase(),
                    &next_word[1..]
                )
                .as_str(),
            )
        {
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
        run_language_tests(Finnish {}, "tests/fi.txt");
    }
}
