use std::sync::LazyLock;

use super::Language;

#[derive(Debug, Clone)]
pub struct Spanish {}
static SPANISH_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/es.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for Spanish {
    fn get_abbreviations(&self) -> &[String] {
        &SPANISH_ABBREVIATIONS
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::tests::run_language_tests;

    #[test]
    fn test_segment() {
        run_language_tests(Spanish {}, "tests/es.txt");
    }
}
