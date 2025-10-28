use super::Language;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Punjabi {}

static ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/pa.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});
impl Language for Punjabi {
    fn get_abbreviations(&self) -> &[String] {
        &ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Punjabi {}, "tests/pa.txt");
    }
}
