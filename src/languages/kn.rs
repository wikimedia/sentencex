use super::Language;
use rustc_hash::FxHashSet;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct Kannada {}

static ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    include_str!("./abbrev/kn.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});
impl Language for Kannada {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Kannada {}, "tests/kn.txt");
    }
}
