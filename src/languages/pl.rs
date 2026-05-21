use super::Language;
use super::parse_abbreviation_list;
use rustc_hash::FxHashSet;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct Polish {}

static ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_abbreviation_list([include_str!("./abbrev/pl.txt")]));
impl Language for Polish {
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
        run_language_tests(Polish {}, "tests/pl.txt");
    }
}
