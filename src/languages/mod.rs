mod en;
mod es;
mod it;
mod ml;
mod pt;
pub use en::English;
pub use es::Spanish;
pub use it::Italian;
pub use ml::Malayalam;
pub use pt::Portuguese;

use regex::Regex;

use crate::constants::EMAIL_REGEX;
use crate::constants::EXCLAMATION_WORDS;
use crate::constants::GLOBAL_SENTENCE_TERMINATORS;
use crate::constants::PARENS_REGEX;
use crate::constants::QUOTES_REGEX;

pub trait Language {
    fn get_sentence_break_regex(&self) -> Regex {
        let pattern = format!("[{}]+", GLOBAL_SENTENCE_TERMINATORS.join(""));
        Regex::new(&pattern).unwrap()
    }

    fn segment(&self, text: &str) -> Vec<String> {
        let mut sentences = Vec::new();
        let paragraphs: Vec<&str> = text.split("\n\n").collect();

        for (pindex, paragraph) in paragraphs.iter().enumerate() {
            if pindex > 0 {
                sentences.push("\n\n".to_string());
            }

            let mut boundaries = vec![0];
            let matches: Vec<(usize, usize)> = self
                .get_sentence_break_regex()
                .find_iter(paragraph)
                .map(|m| (m.start(), m.end()))
                .collect();
            let skippable_ranges = self.get_skippable_ranges(paragraph);

            for (start, end) in matches {
                let mut boundary = self
                    .find_boundary(paragraph, start, end)
                    .unwrap_or(usize::MAX);

                if boundary == usize::MAX {
                    continue;
                }

                let mut in_range = false;
                for (range_start, range_end) in &skippable_ranges {
                    if boundary > *range_start && boundary < *range_end {
                        if boundary + 1 == *range_end && self.is_punctuation_between_quotes() {
                            boundary = *range_end;
                            in_range = false;
                        } else {
                            in_range = true;
                        }
                        break;
                    }
                }

                if in_range {
                    continue;
                }

                boundaries.push(boundary);
            }

            if *boundaries.last().unwrap() != paragraph.len() {
                boundaries.push(paragraph.len());
            }

            for i in 0..boundaries.len() - 1 {
                let start = boundaries[i];
                let end = boundaries[i + 1];

                if start >= paragraph.len() || end > paragraph.len() || start > end {
                    continue;
                }

                let sentence = &paragraph[start..end];
                sentences.push(sentence.to_string());
            }
        }

        sentences
    }

    fn get_abbreviation_char(&self) -> &str {
        "."
    }

    fn get_abbreviations(&self) -> Vec<String> {
        Vec::new()
    }

    fn is_abbreviation(&self, head: &str, _tail: &str, separator: &str) -> bool {
        if self.get_abbreviation_char() != separator {
            return false;
        }

        let last_word = self.get_last_word(head);

        if last_word.is_empty() {
            return false;
        }

        let abbreviations = self.get_abbreviations();
        let is_abbrev = abbreviations.contains(&last_word.to_string());
        let is_abbrev_lower = abbreviations.contains(&last_word.to_lowercase());
        let is_abbrev_upper = abbreviations.contains(&last_word.to_uppercase());

        is_abbrev || is_abbrev_lower || is_abbrev_upper
    }

    fn get_last_word<'a>(&self, text: &'a str) -> &'a str {
        let words: Vec<&str> = text
            .split(|c: char| c.is_whitespace() || c == '.')
            .collect();

        if let Some(&last_word) = words.last() {
            return last_word;
        }
        text
    }

    fn is_exclamation(&self, head: &str, _tail: &str) -> bool {
        let last_word = self.get_last_word(head);
        let exclamation_word = format!("{}!", last_word);
        EXCLAMATION_WORDS.contains(&exclamation_word.as_str())
    }

    fn find_boundary(&self, text: &str, start: usize, end: usize) -> Option<usize> {
        let head = &text[..start];
        let tail = &text[(start + 1)..];

        if let Some(number_ref_match) = crate::constants::NUMBERED_REFERENCE_REGEX.find(tail) {
            return Some(start + 1 + number_ref_match.end());
        }

        if self.continue_in_next_word(tail) {
            return None;
        }

        if self.is_abbreviation(head, tail, &text[start..end]) {
            return None;
        }

        if self.is_exclamation(head, tail) {
            return None;
        }

        if let Some(space_after_sep_match) = crate::constants::SPACE_AFTER_SEPARATOR.find(tail) {
            return Some(start + 1 + space_after_sep_match.end());
        }

        Some(end)
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        let regex = regex::Regex::new(r"^[0-9a-z]").unwrap();
        regex.is_match(text_after_boundary)
    }

    fn get_skippable_ranges(&self, text: &str) -> Vec<(usize, usize)> {
        let mut skippable_ranges = Vec::new();

        for mat in QUOTES_REGEX.find_iter(text) {
            skippable_ranges.push((mat.start(), mat.end()));
        }

        for mat in PARENS_REGEX.find_iter(text) {
            skippable_ranges.push((mat.start(), mat.end()));
        }

        for mat in EMAIL_REGEX.find_iter(text) {
            skippable_ranges.push((mat.start(), mat.end()));
        }

        skippable_ranges
    }
    fn is_punctuation_between_quotes(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    pub fn run_language_tests<T: Language>(language: T, test_file: &str) {
        let content = fs::read_to_string(test_file).expect("Failed to read test file");
        let test_cases: Vec<&str> = content.split("===\n").collect();

        for case in test_cases {
            if case.trim().starts_with('#') {
                continue; // Skip comment lines
            }
            let parts: Vec<&str> = case.split("---\n").collect();
            if parts.len() != 2 {
                continue; // Skip malformed test cases
            }

            let input = parts[0].trim();
            let expected: Vec<&str> = parts[1].lines().map(|line| line.trim()).collect();
            let result = language.segment(input);
            let trimmed_result: Vec<String> =
                result.iter().map(|item| item.trim().to_string()).collect();

            assert_eq!(trimmed_result, expected, "Failed for input: \n{}", input);
        }
    }
}
