use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use super::Language;
use super::parse_abbreviation_list;

#[derive(Debug, Clone)]
pub struct Telugu {}

static TELUGU_ABBREVIATIONS: LazyLock<FxHashSet<String>> = LazyLock::new(|| {
    parse_abbreviation_list([
        include_str!("./abbrev/te.txt"),
        include_str!("./abbrev/en.txt"),
    ])
});
impl Language for Telugu {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
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
