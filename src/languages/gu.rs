use std::sync::LazyLock;

use super::Language;

#[derive(Debug, Clone)]
pub struct Gujarati {}

static GUJARATI_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/gu.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});
impl Language for Gujarati {
    fn get_abbreviations(&self) -> &[String] {
        &GUJARATI_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Gujarati {}, "tests/gu.txt");
    }
}
