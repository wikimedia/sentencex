use super::Language;
use super::trailing_markers::MarkerTable;
use super::{parse_lowercase_word_list, parse_markers_list, parse_word_list};
use regex::Regex;
use rustc_hash::FxHashSet;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct English {}

static ENGLISH_ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_lowercase_word_list([include_str!("./abbrev/en.txt")]));

static ENGLISH_SENTENCE_STARTERS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_word_list([include_str!("./starters/en.txt")]));

static ENGLISH_FRONTING_WORDS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_lowercase_word_list([include_str!("./fronting/en.txt")]));

static ENGLISH_MARKERS: LazyLock<MarkerTable> = LazyLock::new(|| {
    MarkerTable::build(parse_markers_list(include_str!(
        "./trailing_markers/en.txt"
    )))
});

// English `I` is the one capital that case cannot distinguish from a sentence
// start, so after an ellipsis run `... I'm` reads as continuation.
static ENGLISH_ELLIPSIS_I_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+I(?:[\s'\u{2019}]|$)").unwrap());

impl Language for English {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &ENGLISH_ABBREVIATIONS
    }

    fn get_sentence_starters(&self) -> &FxHashSet<String> {
        &ENGLISH_SENTENCE_STARTERS
    }

    fn get_fronting_words(&self) -> &FxHashSet<String> {
        &ENGLISH_FRONTING_WORDS
    }

    fn get_trailing_markers(&self) -> &'static MarkerTable {
        &ENGLISH_MARKERS
    }

    fn is_ellipsis_continuation(&self, text_after_run: &str) -> bool {
        super::language::ELLIPSIS_CONTINUE_REGEX.is_match(text_after_run)
            || ENGLISH_ELLIPSIS_I_REGEX.is_match(text_after_run)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(English {}, "tests/en.txt");
    }
}
