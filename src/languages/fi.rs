use std::sync::LazyLock;

use super::Language;
use super::language::continues_after_boundary;

#[derive(Debug, Clone, Copy)]
pub struct Finnish {}

static FINNISH_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/fi.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

const MONTHS: [&str; 12] = [
    "tammikuu",
    "helmikuu",
    "maaliskuu",
    "huhtikuu",
    "toukokuu",
    "kesäkuu",
    "heinäkuu",
    "elokuu",
    "syyskuu",
    "lokakuu",
    "marraskuu",
    "joulukuu",
];

impl Language for Finnish {
    fn get_abbreviations(&self) -> &[String] {
        &FINNISH_ABBREVIATIONS
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        continues_after_boundary(text_after_boundary, &MONTHS)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Finnish {}, "tests/fi.txt");
    }
}
