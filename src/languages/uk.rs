use super::Language;
use rustc_hash::FxHashSet;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct Ukrainian {}

static PATTERN: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[0-9a-zа-яіїєґ]").expect("Failed to compile regex"));

static ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    include_str!("./abbrev/uk.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

impl Language for Ukrainian {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &ABBREVIATIONS
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        PATTERN.is_match(text_after_boundary)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Ukrainian {}, "tests/uk.txt");
    }
}
