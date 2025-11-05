use std::sync::LazyLock;

use super::Language;

#[derive(Debug, Clone)]
pub struct Arabic {}
static ARABIC_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/ar.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for Arabic {
    fn get_abbreviations(&self) -> &[String] {
        &ARABIC_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Arabic {}, "tests/ar.txt");
    }
}
