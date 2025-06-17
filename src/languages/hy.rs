use regex::Regex;

use crate::constants::GLOBAL_SENTENCE_TERMINATORS;

use super::Language;

#[derive(Debug, Clone)]
pub struct Armenian {}

impl Language for Armenian {
    fn get_abbreviations(&self) -> Vec<String> {
        fn get_abbreviations(&self) -> Vec<String> {
            include_str!("./abbrev/en.txt")
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.starts_with("//") && !line.is_empty())
                .collect()
        }
    }

    fn get_sentence_break_regex(&self) -> Regex {
        let hy_terminators: Vec<&str> = GLOBAL_SENTENCE_TERMINATORS
            .iter()
            .filter(|&&c| c != ".")
            .cloned()
            .collect();
        let pattern = format!("[{};։՜:]]+", hy_terminators.join(""));
        Regex::new(&pattern).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Armenian {}, "tests/hy.txt");
    }
}
