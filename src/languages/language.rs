use regex::Regex;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::SentenceBoundary;
use crate::constants::EMAIL_REGEX;
use crate::constants::EXCLAMATION_WORDS;
use crate::constants::GLOBAL_SENTENCE_TERMINATORS;
use crate::constants::PARENS_REGEX;
use crate::constants::QUOTES_REGEX;

static SENTENCE_BREAK_REGEX_CACHE: LazyLock<Mutex<HashMap<String, Regex>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static CONTINUE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9a-z]").unwrap());

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkippableRangeType {
    Quote,
    Parentheses,
    Email,
}

#[derive(Debug, Clone, Copy)]
pub struct SkippableRange {
    pub start: usize,
    pub end: usize,
    pub range_type: SkippableRangeType,
}

impl SkippableRange {
    pub fn new(start: usize, end: usize, range_type: SkippableRangeType) -> Self {
        Self {
            start,
            end,
            range_type,
        }
    }

    pub fn contains(&self, position: usize) -> bool {
        position > self.start && position < self.end
    }

    pub fn is_quote(&self) -> bool {
        self.range_type == SkippableRangeType::Quote
    }
}

pub trait Language {
    /// Returns a compiled regex pattern that matches sentence terminating punctuation.
    /// This regex is used to identify potential sentence boundaries in text.
    /// The pattern is cached for performance and includes all global sentence terminators
    /// like periods, exclamation marks, and question marks.
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

    /// Analyzes the input text and returns a vector of sentence boundaries.
    /// This is the main method for sentence segmentation that:
    /// 1. Splits text into paragraphs at double newlines
    /// 2. Identifies potential sentence breaks using regex patterns
    /// 3. Filters out false positives (abbreviations, quotes, etc.)
    /// 4. Returns structured boundary information including start/end positions and boundary symbols
    /// Each boundary contains the sentence text, position indices, and metadata about the boundary type.
    fn get_sentence_boundaries<'a>(&self, text: &'a str) -> Vec<SentenceBoundary<'a>> {
        // Pre-allocate boundaries with estimated capacity (rough estimate: 1 sentence per 50 characters)
        let estimated_sentences = (text.len() / 50).max(1);
        let mut boundaries = Vec::with_capacity(estimated_sentences);

        // Split by paragraph breaks (one or more newlines with optional whitespace)
        let para_split_re = Regex::new(r"\n[\r]*\n").unwrap();
        let paragraphs: Vec<&str> = para_split_re.split(text).collect();

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
                    start_byte: paragraph_start,
                    end_byte: paragraph_start + 2,
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

                for range in &skippable_ranges {
                    if range.contains(boundary) {
                        let next_word = self.get_next_word_approx(text, range.end);
                        let boundary_extend = self.get_boundary_extend(next_word);
                        if range.is_quote()
                            && text.ceil_char_boundary(boundary + 1) == range.end
                            && boundary_extend >= 0
                        {
                            boundary = range.end + boundary_extend as usize;
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
                    // Use char_indices for more efficient character iteration
                    paragraph[..end]
                        .char_indices()
                        .next_back()
                        .and_then(|(idx, _)| {
                            let char_str = &paragraph[idx..end];
                            if GLOBAL_SENTENCE_TERMINATORS.contains(&char_str) {
                                Some(char_str.to_string())
                            } else {
                                None
                            }
                        })
                } else {
                    None
                };

                let start_byte = paragraph_start_offset + start;
                let end_byte = paragraph_start_offset + end;

                let start_index = paragraph[..paragraph.floor_char_boundary(start_byte)]
                    .chars()
                    .count();
                let end_index = start_index + sentence_text.chars().count();

                boundaries.push(SentenceBoundary {
                    start_index,
                    end_index,
                    start_byte,
                    end_byte,
                    text: sentence_text,
                    boundary_symbol,
                    is_paragraph_break: false,
                });
            }
        }

        boundaries
    }

    /// Segments the input text into individual sentences and returns them as string slices.
    /// This is a convenience method that builds on get_sentence_boundaries() but returns
    /// only the sentence text content without the additional boundary metadata.
    /// Used when you only need the segmented sentences and not their position information.
    fn segment<'a>(&self, text: &'a str) -> Vec<&'a str> {
        // Pre-allocate with estimated capacity based on text length
        let estimated_sentences = (text.len() / 50).max(1);
        let mut sentences = Vec::with_capacity(estimated_sentences);

        let boundaries = self.get_sentence_boundaries(text);
        for boundary in boundaries {
            if !boundary.text.is_empty() {
                sentences.push(boundary.text);
            }
        }

        sentences
    }

    /// Returns the character used to mark abbreviations in this language.
    /// By default returns "." (period), but should be overridden by specific languages
    /// that use different abbreviation markers. Used by the abbreviation detection logic
    /// to determine if a potential sentence boundary is actually an abbreviation.
    fn get_abbreviation_char(&self) -> &str {
        "."
    }

    /// Returns a list of known abbreviations for this language.
    /// These are used to prevent false sentence breaks at abbreviation periods.
    /// For example, "Dr." or "etc." should not trigger a sentence boundary.
    /// Languages should override this to provide their specific abbreviation lists.
    /// Returns an empty slice by default.
    fn get_abbreviations(&self) -> &[String] {
        &[]
    }

    /// Determines how many characters to extend a boundary when continuing into the next word.
    /// Returns -1 if the word indicates the boundary should not be created (continuation case).
    /// Returns 0 or positive number indicating how many whitespace/punctuation characters
    /// to skip when positioning the boundary. Used to handle cases like quoted sentences
    /// where the boundary should include trailing punctuation and whitespace.
    fn get_boundary_extend(&self, word: &str) -> i8 {
        if self.continue_in_next_word(word.trim()) {
            // not a boundary.
            return -1;
        }

        let mut count = 0i8;
        for ch in word.chars() {
            if ch.is_whitespace() || GLOBAL_SENTENCE_TERMINATORS.contains(&ch.to_string().as_str())
            {
                count += 1;
                if count == i8::MAX {
                    break; // Prevent overflow
                }
            } else {
                break;
            }
        }

        word.ceil_char_boundary(count as usize) as i8
    }

    /// Checks if a potential sentence boundary is actually part of an abbreviation.
    /// Examines the text before the separator to see if it ends with a known abbreviation.
    /// Returns true if this appears to be an abbreviation (and thus not a sentence boundary),
    /// false if it's likely a genuine sentence end. Used to prevent breaking sentences
    /// at abbreviations like "Dr. Smith" or "etc."
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

    /// Extracts the last word from the given text by splitting on whitespace and periods.
    /// Used primarily by abbreviation detection to check if the word before a potential
    /// sentence boundary is a known abbreviation. Returns an empty string if no words
    /// are found. This is a performance-optimized version that avoids collecting all words.
    fn get_last_word<'a>(&self, text: &'a str) -> &'a str {
        // Find the last word without collecting all words
        text.split(|c: char| c.is_whitespace() || c == '.')
            .last()
            .unwrap_or("")
    }

    /// Checks if a potential sentence boundary is actually an exclamation word that shouldn't
    /// trigger a sentence break. Examines the last word before the boundary and checks if
    /// it's in the list of known exclamation words (like "Hey!" or "Wow!").
    /// Returns true if this is an exclamation that should not break the sentence.
    fn is_exclamation(&self, head: &str, _tail: &str) -> bool {
        let last_word = self.get_last_word(head);
        let exclamation_word = format!("{}!", last_word);
        EXCLAMATION_WORDS.contains(&exclamation_word.as_str())
    }

    /// Returns an approximate substring of the next word(s) starting from the given position.
    /// Limited to a maximum of 30 characters for performance. Used to analyze context
    /// after a potential sentence boundary to determine if the boundary should be created.
    /// Handles UTF-8 character boundaries safely to avoid panics on non-ASCII text.
    fn get_next_word_approx<'a>(&self, text: &'a str, start: usize) -> &'a str {
        if start >= text.len() {
            return "";
        }

        let max_chars = 30;
        let safe_start = text.floor_char_boundary(start);
        let end_pos = (start + max_chars).min(text.len());
        &text[safe_start..text.ceil_char_boundary(end_pos)]
    }

    /// Analyzes a potential sentence boundary and determines the exact position where
    /// the sentence should end, or returns None if this shouldn't be a boundary.
    /// Considers abbreviations, exclamations, numbered references, and continuation patterns.
    /// This is the core logic that distinguishes true sentence boundaries from false positives
    /// like abbreviations or mid-sentence punctuation.
    fn find_boundary(&self, text: &str, start: usize, end: usize) -> Option<usize> {
        let head = &text[..start];
        let next_index = text.ceil_char_boundary(start + 1);

        let next_word_approx = self.get_next_word_approx(text, next_index);

        if let Some(number_ref_match) =
            crate::constants::NUMBERED_REFERENCE_REGEX.find(next_word_approx)
        {
            return Some(next_index + number_ref_match.end());
        }

        if self.continue_in_next_word(next_word_approx) {
            return None;
        }

        if self.is_abbreviation(head, next_word_approx, &text[start..end]) {
            return None;
        }

        if self.is_exclamation(head, next_word_approx) {
            return None;
        }

        if let Some(space_after_sep_match) =
            crate::constants::SPACE_AFTER_SEPARATOR.find(next_word_approx)
        {
            return Some(next_index + space_after_sep_match.end());
        }

        Some(end)
    }

    /// Determines if the text after a potential boundary indicates the sentence should continue.
    /// Returns true if the next word starts with a lowercase letter or number, suggesting
    /// the sentence is continuing rather than starting a new one. This helps avoid breaking
    /// sentences at abbreviations or in the middle of compound sentences.
    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        CONTINUE_REGEX.is_match(text_after_boundary)
    }

    /// Identifies ranges of text that should be skipped during sentence boundary detection.
    /// This includes quoted text, parenthetical expressions, and email addresses where
    /// internal punctuation should not trigger sentence breaks. Returns a sorted vector
    /// of ranges that can be efficiently checked during boundary detection to avoid
    /// false positives within these special text regions.
    fn get_skippable_ranges(&self, text: &str) -> Vec<SkippableRange> {
        // Pre-allocate with estimated capacity based on text length (rough estimate: 1 range per 200 characters)
        let estimated_ranges = (text.len() / 200).max(1);
        let mut skippable_ranges = Vec::with_capacity(estimated_ranges);

        for mat in QUOTES_REGEX.find_iter(text) {
            skippable_ranges.push(SkippableRange::new(
                mat.start(),
                mat.end(),
                SkippableRangeType::Quote,
            ));
        }

        for mat in PARENS_REGEX.find_iter(text) {
            skippable_ranges.push(SkippableRange::new(
                mat.start(),
                mat.end(),
                SkippableRangeType::Parentheses,
            ));
        }

        for mat in EMAIL_REGEX.find_iter(text) {
            skippable_ranges.push(SkippableRange::new(
                mat.start(),
                mat.end(),
                SkippableRangeType::Email,
            ));
        }

        // Sort ranges by start position for more efficient lookups
        skippable_ranges.sort_unstable_by_key(|r| r.start);
        skippable_ranges
    }
}
