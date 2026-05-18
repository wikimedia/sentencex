use super::Language;
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct English {}

static ENGLISH_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    include_str!("./abbrev/en.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});

// English `I` is the one capital that case cannot distinguish from a sentence
// start, so after an ellipsis run `... I'm` reads as continuation.
static ENGLISH_ELLIPSIS_I_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+I(?:[\s'\u{2019}]|$)").unwrap());

impl Language for English {
    fn get_abbreviations(&self) -> &[String] {
        &ENGLISH_ABBREVIATIONS
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
