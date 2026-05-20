use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;

#[derive(Debug, Clone)]
pub struct Amharic {}
static AMHARIC_ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    include_str!("./abbrev/am.txt")
        .lines()
        .chain(include_str!("./abbrev/am.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for Amharic {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &AMHARIC_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Amharic {}, "tests/am.txt");
    }
}
