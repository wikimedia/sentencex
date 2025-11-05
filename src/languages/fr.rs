use std::sync::LazyLock;

use super::Language;

#[derive(Debug, Clone)]
pub struct French {}
static FRENCH_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/fr.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for French {
    fn get_abbreviations(&self) -> &[String] {
        &FRENCH_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(French {}, "tests/fr.txt");
    }
}
