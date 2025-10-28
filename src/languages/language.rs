use regex::Regex;

use crate::SentenceBoundary;
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

    fn get_sentence_boundaries<'a>(&self, text: &'a str) -> Vec<SentenceBoundary<'a>> {
        let mut boundaries = Vec::new();
        let paragraphs: Vec<&str> = text.split("\n\n").collect();

        for (pindex, paragraph) in paragraphs.iter().enumerate() {
            if pindex > 0 {
                let paragraph_start =
                    paragraphs[..pindex].iter().map(|p| p.len()).sum::<usize>() + (pindex * 2);
                boundaries.push(SentenceBoundary {
                    start_index: paragraph_start,
                    end_index: paragraph_start + 2,
                    text: "\n\n",
                    boundary_symbol: None,
                    is_paragraph_break: true,
                });
            }

            let paragraph_start_offset = if pindex == 0 {
                0
            } else {
                paragraphs[..pindex].iter().map(|p| p.len()).sum::<usize>() + (pindex * 2) + 2
            };

            let mut sentence_boundaries = vec![0];
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

                sentence_boundaries.push(boundary);
            }

            if *sentence_boundaries.last().unwrap() != paragraph.len() {
                sentence_boundaries.push(paragraph.len());
            }

            for i in 0..sentence_boundaries.len() - 1 {
                let start = sentence_boundaries[i];
                let end = sentence_boundaries[i + 1];

                if start >= paragraph.len() || end > paragraph.len() || start > end {
                    continue;
                }

                let sentence_text = &paragraph[start..end];
                let boundary_symbol = if end < paragraph.len() {
                    // Find the last character before the boundary, accounting for multibyte characters
                    let chars: Vec<char> = paragraph[..end].chars().collect();

                    if let Some(&last_char) = chars.last() {
                        let last_char_str = last_char.to_string();
                        if GLOBAL_SENTENCE_TERMINATORS.contains(&last_char_str.as_str()) {
                            Some(last_char_str)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                boundaries.push(SentenceBoundary {
                    start_index: paragraph_start_offset + start,
                    end_index: paragraph_start_offset + end,
                    text: sentence_text,
                    boundary_symbol,
                    is_paragraph_break: false,
                });
            }
        }

        boundaries
    }

    fn segment(&self, text: &str) -> Vec<String> {
        self.get_sentence_boundaries(text)
            .into_iter()
            .map(|boundary| boundary.text.to_string())
            .collect()
    }

    fn get_abbreviation_char(&self) -> &str {
        "."
    }

    fn get_abbreviations(&self) -> &[String] {
        &[]
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
        let mut next = start + 1;

        while !text.is_char_boundary(next) {
            // Move forward till next char boundary. Applies for multibyte unicode chars
            next += 1;
        }
        let tail = &text[next..];

        if let Some(number_ref_match) = crate::constants::NUMBERED_REFERENCE_REGEX.find(tail) {
            return Some(next + number_ref_match.end());
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
            return Some(next + space_after_sep_match.end());
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
