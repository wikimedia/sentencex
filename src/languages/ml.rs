use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_abbreviation_list;

#[derive(Debug, Clone)]
pub struct Malayalam {}
static MALAYALAM_ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    parse_abbreviation_list([
        include_str!("./abbrev/ml.txt"),
        include_str!("./abbrev/en.txt"),
    ])
});

impl Language for Malayalam {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &MALAYALAM_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Malayalam {}, "tests/ml.txt");
    }
}
