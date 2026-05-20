use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use regex::Regex;

use super::Language;
use super::parse_word_list;

#[derive(Debug, Clone)]
pub struct Kazakh {}

static KAZAKH_ABBREVIATIONS: LazyLock<FxHashSet<String>> =
    LazyLock::new(|| parse_word_list([include_str!("./abbrev/kk.txt")]));

// Extends the base continuation regex with Cyrillic lowercase range (а-я).
static KAZAKH_CONTINUE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\W*[0-9a-zа-я]").unwrap());

impl Language for Kazakh {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        &KAZAKH_ABBREVIATIONS
    }

    fn get_last_word<'a>(&self, text: &'a str) -> &'a str {
        text.split(|c: char| c.is_whitespace() || c == '.')
            .rfind(|word| !word.is_empty())
            .unwrap_or("")
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        KAZAKH_CONTINUE_REGEX.is_match(text_after_boundary)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Kazakh {}, "tests/kk.txt");
    }
}
