use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::language::continues_after_boundary;
use super::parse_abbreviation_list;

#[derive(Debug, Clone)]
pub struct Finnish {}

static FINNISH_ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_abbreviation_list([include_str!("./abbrev/fi.txt")]));

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
    fn get_abbreviations(&self) -> &FxHashSet<String> {
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
