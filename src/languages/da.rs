use once_cell::sync::Lazy;
use regex::Regex;

use super::Language;

#[derive(Debug, Clone)]
pub struct Danish {}
static DANISH_ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/da.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for Danish {
    fn get_abbreviations(&self) -> Vec<String> {
        DANISH_ABBREVIATIONS.clone()
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        let regex = Regex::new(r"^\W*[0-9a-z]").unwrap();
        regex.is_match(text_after_boundary)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Danish {}, "tests/da.txt");
    }
}
