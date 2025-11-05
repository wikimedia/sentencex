use std::sync::LazyLock;

use super::Language;

#[derive(Debug, Clone)]
pub struct Hindi {}
static HINDI_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/hi.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for Hindi {
    fn get_abbreviations(&self) -> &[String] {
        &HINDI_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Hindi {}, "tests/hi.txt");
    }
}
