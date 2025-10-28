use once_cell::sync::Lazy;

use super::Language;

#[derive(Debug, Clone)]
pub struct Telegu {}

static TELEGU_ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/te.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});
impl Language for Telegu {
    fn get_abbreviations(&self) -> &[String] {
        &TELEGU_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Telegu {}, "tests/te.txt");
    }
}
