use once_cell::sync::Lazy;

use super::Language;

#[derive(Debug, Clone)]
pub struct Gujarati {}

static GUJARATI_ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/gu.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});
impl Language for Gujarati {
    fn get_abbreviations(&self) -> Vec<String> {
        GUJARATI_ABBREVIATIONS.clone()
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
