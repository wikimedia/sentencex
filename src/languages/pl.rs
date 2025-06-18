use super::Language;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Polish {}

static ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/pl.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});
impl Language for Polish {
    fn get_abbreviations(&self) -> Vec<String> {
        ABBREVIATIONS.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Polish {}, "tests/pl.txt");
    }
}
