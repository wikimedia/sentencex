use crate::constants::ROMAN_NUMERALS;

use super::Language;

#[derive(Debug, Clone)]
pub struct Portuguese {}

impl Language for Portuguese {
    fn get_abbreviations(&self) -> Vec<String> {
        let mut abbreviations: Vec<String> = include_str!("./abbrev/pt.abbreviations.txt")
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.starts_with("//") && !line.is_empty())
            .collect();

        abbreviations.extend(ROMAN_NUMERALS.iter().map(|&s| s.to_string()));
        abbreviations.extend(ROMAN_NUMERALS.iter().map(|&s| s.to_uppercase()));
        abbreviations
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
