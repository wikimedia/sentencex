use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_abbreviation_list;

#[derive(Debug, Clone)]
pub struct Spanish {}
static SPANISH_ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_abbreviation_list([include_str!("./abbrev/es.txt")]));

impl Language for Spanish {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &SPANISH_ABBREVIATIONS
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::languages::tests::run_language_tests;

    #[test]
    fn test_segment() {
        run_language_tests(Spanish {}, "tests/es.txt");
    }
}
