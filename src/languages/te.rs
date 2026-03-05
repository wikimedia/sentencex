use std::sync::LazyLock;

use super::Language;

#[derive(Debug, Clone)]
pub struct Telugu {}

static TELUGU_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/te.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});
impl Language for Telugu {
    fn get_abbreviations(&self) -> &[String] {
        &TELUGU_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Telugu {}, "tests/te.txt");
    }
}
