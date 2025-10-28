use once_cell::sync::Lazy;

use super::Language;

#[derive(Debug, Clone)]
pub struct Bengali {}
static BENGALI_ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/bn.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for Bengali {
    fn get_abbreviations(&self) -> &[String] {
        &BENGALI_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Bengali {}, "tests/bn.txt");
    }
}
