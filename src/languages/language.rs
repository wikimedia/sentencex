use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::SentenceBoundary;
use crate::constants::EMAIL_REGEX;
use crate::constants::EXCLAMATION_WORDS;
use crate::constants::GLOBAL_SENTENCE_TERMINATORS;
use crate::constants::PARENS_REGEX;
use crate::constants::QUOTES_REGEX;

lazy_static::lazy_static! {
    static ref SENTENCE_BREAK_REGEX_CACHE: Mutex<HashMap<String, Regex>> = Mutex::new(HashMap::new());
    static ref CONTINUE_REGEX: Regex = Regex::new(r"^[0-9a-z]").unwrap();
}

pub trait Language {
    fn get_sentence_break_regex(&self) -> Regex {
        let pattern = format!("[{}]+", GLOBAL_SENTENCE_TERMINATORS.join(""));

        // Try to get from cache first
        {
            let cache = SENTENCE_BREAK_REGEX_CACHE.lock().unwrap();
            if let Some(regex) = cache.get(&pattern) {
                return regex.clone();
            }
        }

        // Create new regex and cache it
        let regex = Regex::new(&pattern).unwrap();
        {
            let mut cache = SENTENCE_BREAK_REGEX_CACHE.lock().unwrap();
            cache.insert(pattern, regex.clone());
        }

        regex
    }

    fn get_sentence_boundaries<'a>(&self, text: &'a str) -> Vec<SentenceBoundary<'a>> {
        // Pre-allocate boundaries with estimated capacity (rough estimate: 1 sentence per 50 characters)
        let estimated_sentences = (text.len() / 50).max(1);
        let mut boundaries = Vec::with_capacity(estimated_sentences);

        let paragraphs: Vec<&str> = text.split("\n\n").collect();

        // Pre-calculate all paragraph offsets in one pass
        let mut paragraph_offsets = Vec::with_capacity(paragraphs.len());
        let mut current_offset = 0;
        for (i, paragraph) in paragraphs.iter().enumerate() {
            paragraph_offsets.push(current_offset);
            current_offset += paragraph.len();
            if i < paragraphs.len() - 1 {
                current_offset += 2; // for "\n\n"
            }
        }

        // Pre-allocate sentence_boundaries once and reuse for all paragraphs
        let estimated_paragraph_sentences = 10; // reasonable default for typical paragraphs
        let mut sentence_boundaries = Vec::with_capacity(estimated_paragraph_sentences);

        for (pindex, paragraph) in paragraphs.iter().enumerate() {
            if pindex > 0 {
                let paragraph_start = paragraph_offsets[pindex];
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
                paragraph_offsets[pindex] + 2
            };
            sentence_boundaries.clear();
            sentence_boundaries.push(0);

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
                let boundary_symbol = if end > 0 && end <= paragraph.len() {
                    // Work backwards to find the last character without collecting all chars
                    let mut char_end = end;
                    while char_end > 0 && !paragraph.is_char_boundary(char_end) {
                        char_end -= 1;
                    }

                    if char_end > 0 {
                        // Find start of the character
                        let mut char_start = char_end - 1;
                        while char_start > 0 && !paragraph.is_char_boundary(char_start) {
                            char_start -= 1;
                        }

                        let last_char_str = &paragraph[char_start..char_end];
                        if GLOBAL_SENTENCE_TERMINATORS.contains(&last_char_str) {
                            Some(last_char_str.to_string())
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

    fn segment<'a>(&self, text: &'a str) -> Vec<&'a str> {
        // Pre-allocate with estimated capacity based on text length
        let estimated_sentences = (text.len() / 50).max(1);
        let mut sentences = Vec::with_capacity(estimated_sentences);

        let boundaries = self.get_sentence_boundaries(text);
        for boundary in boundaries {
            sentences.push(boundary.text);
        }

        sentences
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
        // Find the last word without collecting all words
        text.split(|c: char| c.is_whitespace() || c == '.')
            .last()
            .unwrap_or("")
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
        CONTINUE_REGEX.is_match(text_after_boundary)
    }

    fn get_skippable_ranges(&self, text: &str) -> Vec<(usize, usize)> {
        // Pre-allocate with estimated capacity based on text length (rough estimate: 1 range per 200 characters)
        let estimated_ranges = (text.len() / 200).max(1);
        let mut skippable_ranges = Vec::with_capacity(estimated_ranges);

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
