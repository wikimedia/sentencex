use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use regex::Regex;

use super::language::DEFAULT_SENTENCE_BREAK_PATTERN;
use super::{English, Language};

#[derive(Debug, Clone)]
pub struct Thai {}

// Thai has no sentence-ending punctuation: a sentence or clause ends with a
// space before the next word. Besides punctuation, a run of spaces followed by
// the first letter of a Thai word (a consonant or a leading vowel) is a
// candidate, and `find_boundary` checks the letter before it.
static THAI_SENTENCE_BREAK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!(
        "{}|[ \t]+[\u{0E01}-\u{0E2E}\u{0E40}-\u{0E44}]",
        *DEFAULT_SENTENCE_BREAK_PATTERN
    );
    Regex::new(&pattern).unwrap()
});

// Conjunctions that join clauses. Thai writes a space before them inside a
// sentence, so a space before one does not end the sentence.
const CONJUNCTIONS: [&str; 5] = [
    "และ",   // and
    "หรือ",   // or
    "แต่",    // but
    "เพราะ", // because
    "ซึ่ง",    // which
];

/// Whether a Thai word can end with `c`: a consonant, vowel or tone mark. ฯ
/// (abbreviation) and ๆ (repetition) are left out, since a space follows them
/// inside a sentence too.
fn ends_thai_word(c: char) -> bool {
    matches!(
        c,
        '\u{0E01}'..='\u{0E2E}' | '\u{0E30}'..='\u{0E3A}' | '\u{0E40}'..='\u{0E45}' | '\u{0E47}'..='\u{0E4E}'
    )
}

// Punctuation follows the English rules, which Thai used before it had its own.
impl Language for Thai {
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        English {}.get_abbreviations()
    }

    fn get_sentence_starters(&self) -> &FxHashSet<String> {
        English {}.get_sentence_starters()
    }

    fn get_sentence_break_regex(&self) -> &'static Regex {
        &THAI_SENTENCE_BREAK_REGEX
    }

    /// A space ends a sentence only between two Thai words, and not before a
    /// conjunction. A space next to digits or non-Thai words stays inside the
    /// sentence.
    fn find_boundary(&self, text: &str, start: usize, end: usize) -> Option<usize> {
        if !text[start..end].starts_with([' ', '\t']) {
            return English {}.find_boundary(text, start, end);
        }

        // The match ends with the first letter of the next word.
        let (next_word, _) = text[..end].char_indices().next_back()?;
        let before = text[..start].chars().next_back()?;
        let joins_clause = CONJUNCTIONS
            .iter()
            .any(|word| text[next_word..].starts_with(word));
        (ends_thai_word(before) && !joins_clause).then_some(next_word)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Thai {}, "tests/th.txt");
    }

    #[test]
    fn test_segment_text_without_thai_like_english() {
        run_language_tests(Thai {}, "tests/en.txt");
    }
}
