use regex::Regex;
use std::sync::LazyLock;

use crate::SentenceBoundary;
use crate::constants::EMAIL_REGEX;
use crate::constants::EXCLAMATION_WORDS;
use crate::constants::GLOBAL_SENTENCE_TERMINATORS;
use crate::constants::PARENS_REGEX;
use crate::constants::QUOTE_CLOSERS_BY_LEN;
use crate::constants::QUOTE_PAIRS;
use crate::constants::QUOTES_REGEX;
use crate::constants::SPACE_AFTER_SEPARATOR;

static DEFAULT_SENTENCE_BREAK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Branch 1 (`\.(?:[ \t]+\.){2,}`) coalesces three-or-more spaced dots
    // (`. . .`, `. . . .`) into one match. Two-dot `. .` is excluded so a
    // period followed by a leading ellipsis (`raak. ...en`) is not eaten as a
    // single run. `[ \t]` (not `\s`) keeps newlines intact for paragraph splits.
    //
    // Branch 2 (`[!?…](?:[ \t]+[!?…])+`) coalesces two-or-more spaced runs,
    // mixed or homogeneous (`! !`, `? ? ?`, `! ?`, `… !`). `+` rather than
    // `{2,}` is safe here — there's no leading-ellipsis equivalent for `!`/`?`.
    // Both branches must precede the class for leftmost-first alternation.
    let pattern = format!(
        r"\.(?:[ \t]+\.){{2,}}|[!?…](?:[ \t]+[!?…])+|[{}]+",
        GLOBAL_SENTENCE_TERMINATORS.join("")
    );

    Regex::new(&pattern).unwrap()
});

static CONTINUE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9a-z]").unwrap());

// Matches a lowercase letter or digit, optionally preceded by non-word characters
// (e.g. a space or punctuation). Used by languages that extend the base continuation
// check with their own month lists.
static CONTINUE_AFTER_NONWORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\W*[0-9a-z]").unwrap());

// Continuation rule for ellipsis matches: treat the run as mid-sentence when
// the follow-up is whitespace + a lowercase/digit (`... no`, `. . . what`) or
// whitespace + a standalone `I` (`... I'm`, `. . . I didn't`). `I` is the one
// capital that can't be distinguished from a sentence start by case, so it's
// folded in as continuation; `No`, `Then`, `And` still mark a boundary.
static ELLIPSIS_CONTINUE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+(?:[0-9a-z]|I(?:[\s'\u{2019}]|$))").unwrap());

// Glued lowercase after an ellipsis (`mean...see`) is intra-utterance hesitation
// and continues the sentence. Only fires when the dots are also glued behind -
// a free-standing leading ellipsis (`raak. ...en`) keeps its boundary.
static ELLIPSIS_GLUED_CONTINUE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9a-z]").unwrap());

static PARA_SPLIT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n[\r]*\n").unwrap());

/// True when `closer` is the closing token of a symmetric quote pair —
/// `'`, `''`, `"`, etc., where opener equals closer. These pairs are
/// inherently ambiguous: `QUOTES_REGEX` can fail to pair them when they're
/// space-padded, so callers need extra logic to decide orphanhood.
fn is_symmetric_quote_closer(closer: &str) -> bool {
    QUOTE_PAIRS
        .iter()
        .any(|p| p.open == p.close && p.close == closer)
}

/// True when the `closer` token at `boundary` in `paragraph` is an orphan —
/// either it has no matching opener earlier in the paragraph, or it is a
/// lone stray with no counterpart anywhere.
///
/// Asymmetric closers (`»`, `”`, …) are unambiguous and always orphan once
/// `QUOTES_REGEX` has had a chance to pair them. For symmetric closers we
/// inspect the closer's distribution across the paragraph, ignoring any
/// occurrence already consumed by an asymmetric quote pair.
fn is_orphan_closer(
    paragraph: &str,
    boundary: usize,
    closer: &str,
    skippable_ranges: &[SkippableRange],
) -> bool {
    if !is_symmetric_quote_closer(closer) {
        return true;
    }

    let consumed_by_asymmetric_pair = |idx: usize| {
        skippable_ranges
            .iter()
            .any(|r| r.is_quote() && idx >= r.start && idx < r.end)
    };

    let (mut before, mut at_or_after) = (0usize, 0usize);
    for (idx, _) in paragraph.match_indices(closer) {
        if consumed_by_asymmetric_pair(idx) {
            continue;
        }
        if idx < boundary {
            before += 1;
        } else {
            at_or_after += 1;
        }
    }

    let unmatched_opener_before = before % 2 == 1;
    let lone_stray_at_boundary = before == 0 && at_or_after == 1;
    unmatched_opener_before || lone_stray_at_boundary
}

/// True when `range` is a quote range whose opener and closer are the same
/// token — `''…''`, `'…'`, `"…"`, etc. These pairs are inherently ambiguous
/// for the regex pairer and need extra scrutiny when they appear to veto a
/// sentence boundary.
fn is_symmetric_quote_range(text: &str, range: &SkippableRange) -> bool {
    if !range.is_quote() {
        return false;
    }
    let span = &text[range.start..range.end];
    QUOTE_PAIRS.iter().filter(|p| p.open == p.close).any(|p| {
        span.len() >= 2 * p.open.len() && span.starts_with(p.open) && span.ends_with(p.close)
    })
}

/// True when the paragraph contains an odd number of the symmetric quote
/// token that opens `range`. An odd count guarantees at least one orphan
/// occurrence — and when that orphan sits earlier than a real downstream
/// opener, `QUOTES_REGEX` will mispair across a real sentence break. Even
/// counts are structurally consistent and should be trusted.
fn symmetric_token_count_is_odd(paragraph: &str, range: &SkippableRange) -> bool {
    let Some(pair) = QUOTE_PAIRS
        .iter()
        .filter(|p| p.open == p.close)
        .find(|p| paragraph[range.start..].starts_with(p.open))
    else {
        return false;
    };

    paragraph.matches(pair.open).count() % 2 == 1
}

/// True when `quote` and any parens range in `ranges` partially overlap —
/// one endpoint inside, the other outside. Full containment in either
/// direction (a quote wrapping parens, or parens wrapping a quote) is fine
/// and returns false. Partial overlap is the signature of a greedy
/// symmetric-pair pairing that crossed what's really a sentence break.
fn quote_partially_overlaps_parens(quote: &SkippableRange, ranges: &[SkippableRange]) -> bool {
    ranges
        .iter()
        .filter(|r| r.range_type == SkippableRangeType::Parentheses)
        .any(|p| {
            (quote.start < p.start && p.start < quote.end && quote.end < p.end)
                || (p.start < quote.start && quote.start < p.end && p.end < quote.end)
        })
}

/// Push `boundary` only if it advances past the last recorded position.
/// Quote-extension can move a boundary past later regex matches in the same
/// paragraph; this keeps the boundary list strictly increasing.
/// Assumes the caller is passing a non empty list of boundaries.
fn push_if_increasing(boundaries: &mut Vec<usize>, boundary: usize) {
    debug_assert!(!boundaries.is_empty());

    if boundary > *boundaries.last().unwrap() {
        boundaries.push(boundary);
    }
}

/// Find terminator-run matches in `text`, folding a whitespace-separated
/// dot-only follow-up onto a preceding `!`/`?`/`…` run so `Bravo ! .` and
/// `Happy! . . . no one …` surface as one coalesced terminator.
///
/// The regex already coalesces homogeneous runs (`! !`, `. . .`) and
/// contiguous mixed runs like `! ...` (matched by the contiguous-class
/// branch as `!...`). Spaced mixed runs like `! . . .` can't be expressed
/// without lookahead - `[!?…][ \t]+\.` would eat the first dot of an
/// ellipsis - so they arrive here as two matches and are folded only when
/// the follow-up is pure `.`s separated by whitespace.
fn find_terminator_matches(text: &str, regex: &Regex) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();

    for m in regex.find_iter(text) {
        let (start, end) = (m.start(), m.end());

        if let Some(last) = out.last_mut() {
            let prev = &text[last.0..last.1];
            let candidate = &text[start..end];
            let gap = &text[last.1..start];

            let is_blank = |c: char| matches!(c, ' ' | '\t');
            let prev_is_emphatic = prev.ends_with(['!', '?', '…']);
            let candidate_is_dot_run =
                candidate.starts_with('.') && candidate.chars().all(|c| c == '.' || is_blank(c));
            let separated_by_blanks = !gap.is_empty() && gap.chars().all(is_blank);

            if prev_is_emphatic && candidate_is_dot_run && separated_by_blanks {
                last.1 = end;
                continue;
            }
        }

        out.push((start, end));
    }

    out
}

/// Shared helper for languages that continue sentences before month names.
///
/// Returns `true` if `text` starts with a lowercase letter/digit (after optional
/// non-word characters), or if its first whitespace-delimited word (case-insensitively
/// capitalised) is one of the supplied `months`.
pub fn continues_after_boundary(text: &str, months: &[&str]) -> bool {
    if CONTINUE_AFTER_NONWORD_REGEX.is_match(text) {
        return true;
    }

    let next_word = text
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches(['.', '!', '?']);

    if next_word.is_empty() {
        return false;
    }

    // Build a version with the first character upper-cased (handles non-ASCII safely).
    let capitalized: String = next_word
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect();

    months.contains(&next_word) || months.contains(&capitalized.as_str())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkippableRangeType {
    Quote,
    Parentheses,
    Email,
    ListItem,
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

    /// True if `boundary` lies just before this quote's closing mark — i.e. the
    /// sentence terminator sits inside the quoted span with only the closer left.
    /// Opinionated behaviour, but it can help to resolve some real world cases with erroneous
    /// or ambiguous punctuation placement.
    ///
    /// Example: in (start of line) `He said "Hello."`, the `.` boundary is immediately followed
    /// by the closing `"`, so the sentence should extend past the quote rather than break
    /// between the `.` and the `"`.
    pub fn is_inner_terminator(&self, text: &str, boundary: usize) -> bool {
        if !self.is_quote() || boundary >= self.end {
            return false;
        }

        let head = &text[..self.end];
        QUOTE_CLOSERS_BY_LEN
            .iter()
            .any(|c| head.ends_with(*c) && boundary + c.len() == self.end)
    }
}

pub trait Language {
    /// Returns a reference to the compiled regex pattern that matches sentence terminating
    /// punctuation. The default implementation uses a static LazyLock for zero-cost access.
    fn get_sentence_break_regex(&self) -> &'static Regex {
        &DEFAULT_SENTENCE_BREAK_REGEX
    }

    /// Analyzes the input text and returns a vector of sentence boundaries.
    /// This is the main method for sentence segmentation that:
    /// 1. Splits text into paragraphs at double newlines
    /// 2. Identifies potential sentence breaks using regex patterns
    /// 3. Filters out false positives (abbreviations, quotes, etc.)
    /// 4. Returns structured boundary information including start/end positions and boundary symbols
    ///
    /// Each boundary contains the sentence text, position indices, and metadata about the boundary type.
    fn get_sentence_boundaries<'a>(&self, text: &'a str) -> Vec<SentenceBoundary<'a>> {
        // Pre-allocate boundaries with estimated capacity (rough estimate: 1 sentence per 50 characters)
        let estimated_sentences = (text.len() / 50).max(1);
        let mut boundaries = Vec::with_capacity(estimated_sentences);

        // Split by paragraph breaks (one or more newlines with optional whitespace)
        let paragraphs: Vec<&str> = PARA_SPLIT_REGEX.split(text).collect();

        // Pre-calculate all paragraph offsets in one pass
        // CRITICAL: We track both byte offsets AND character offsets separately.
        // This is essential for correct handling of multi-byte UTF-8 characters (e.g., CJK, emoji).
        //
        // - `paragraph_offsets`: byte indices into the original text (for slicing with &text[start..end])
        // - `paragraph_char_offsets`: character counts (for SentenceBoundary.start_index/end_index)
        //
        // Example: "日本語" is 3 characters but 9 bytes in UTF-8:
        //   - byte offset: 0..9
        //   - char offset: 0..3
        let mut paragraph_offsets = Vec::with_capacity(paragraphs.len());
        let mut current_offset = 0;
        let mut paragraph_char_offsets = Vec::with_capacity(paragraphs.len());
        let mut current_char_offset = 0;
        for (i, paragraph) in paragraphs.iter().enumerate() {
            paragraph_offsets.push(current_offset);
            paragraph_char_offsets.push(current_char_offset);
            current_offset += paragraph.len();
            current_char_offset += paragraph.chars().count();
            if i < paragraphs.len() - 1 {
                current_offset += 2; // for "\n\n" bytes
                current_char_offset += 2; // for "\n\n" chars (always 2, regardless of encoding)
            }
        }

        // Pre-allocate sentence_boundaries once and reuse for all paragraphs
        let estimated_paragraph_sentences = 10; // reasonable default for typical paragraphs
        let mut sentence_boundaries = Vec::with_capacity(estimated_paragraph_sentences);
        let sentence_break_regex = self.get_sentence_break_regex();

        for (pindex, paragraph) in paragraphs.iter().enumerate() {
            if pindex > 0 {
                let paragraph_start = paragraph_offsets[pindex];
                let paragraph_char_start = paragraph_char_offsets[pindex];
                boundaries.push(SentenceBoundary {
                    start_index: paragraph_char_start - 2,
                    end_index: paragraph_char_start,
                    start_byte: paragraph_start - 2,
                    end_byte: paragraph_start,
                    text: "\n\n",
                    boundary_symbol: None,
                    is_paragraph_break: true,
                });
            }

            let paragraph_start_offset = if pindex == 0 {
                0
            } else {
                paragraph_offsets[pindex]
            };

            let paragraph_start_char_offset = if pindex == 0 {
                0
            } else {
                paragraph_char_offsets[pindex]
            };

            sentence_boundaries.clear();
            sentence_boundaries.push(0);

            let matches = find_terminator_matches(paragraph, sentence_break_regex);
            let mut skippable_ranges = self.get_skippable_ranges(paragraph);

            // Detect list-item line starts once per paragraph and reuse the
            // result for both atomic-item ranges (so terminator-driven boundaries
            // inside an item are dropped) and explicit boundary emission below.
            let list_starts = super::list_markers::detect_list_items(paragraph);

            if !list_starts.is_empty() {
                for window in list_starts.windows(2) {
                    skippable_ranges.push(SkippableRange::new(
                        window[0],
                        window[1],
                        SkippableRangeType::ListItem,
                    ));
                }

                let last = *list_starts.last().unwrap();

                skippable_ranges.push(SkippableRange::new(
                    last,
                    paragraph.len(),
                    SkippableRangeType::ListItem,
                ));

                // Consumers of skippable_ranges don't seem to rely on order, but I'm not sure if this is
                // intentional or incidental.  If it's intentional, we can drop this sort.
                skippable_ranges.sort_unstable_by_key(|r| r.start);
            }

            'next_match: for (start, end) in matches {
                let Some(mut boundary) = self.find_boundary(paragraph, start, end) else {
                    continue;
                };

                for range in &skippable_ranges {
                    if !range.contains(boundary) {
                        continue;
                    }

                    // Symmetric-pair quote ranges (`''…''`, `'…'`, `"…"`) are
                    // greedy: when a paragraph has more than one closer the
                    // regex can pair across what's really a sentence break.
                    // Detect that mispairing structurally — the range's
                    // endpoints straddle a parens boundary, or the paragraph
                    // has an odd count of the token and there's a strong
                    // sentence break here — and let the boundary through
                    // instead of vetoing it.
                    if is_symmetric_quote_range(paragraph, range)
                        && (quote_partially_overlaps_parens(range, &skippable_ranges)
                            || (symmetric_token_count_is_odd(paragraph, range)
                                && self.has_strong_sentence_break(paragraph, start, end)))
                    {
                        continue;
                    }

                    // Inside a quoted/parens/email range. Either advance past the
                    // closer (if the boundary sits at an inner terminator) or drop
                    // this match entirely. Either way, no further extension applies.
                    if range.is_inner_terminator(paragraph, boundary) {
                        let next_word = self.get_next_word_approx(paragraph, range.end);
                        let extend = self.get_boundary_extend(next_word);
                        if extend >= 0 {
                            push_if_increasing(
                                &mut sentence_boundaries,
                                range.end + extend as usize,
                            );
                        }
                    }
                    continue 'next_match;
                }

                boundary = self.extend_past_orphan_closer(paragraph, boundary, &skippable_ranges);
                push_if_increasing(&mut sentence_boundaries, boundary);
            }

            // Merge in list-item line starts as sentence boundaries. They may
            // interleave with terminator boundaries in source order, so we
            // sort + dedup once rather than maintain the increasing invariant
            // during insertion.
            if !list_starts.is_empty() {
                for &start in &list_starts {
                    if start > 0 {
                        sentence_boundaries.push(start);
                    }
                }

                sentence_boundaries.sort_unstable();
                sentence_boundaries.dedup();
            }

            if *sentence_boundaries.last().unwrap() != paragraph.len() {
                sentence_boundaries.push(paragraph.len());
            }

            let mut prev_end_index = paragraph_start_char_offset;
            let mut prev_end_byte = 0;

            for i in 0..sentence_boundaries.len() - 1 {
                let start = sentence_boundaries[i];
                let end = sentence_boundaries[i + 1];

                if start >= paragraph.len() || end > paragraph.len() || start > end {
                    continue;
                }

                let sentence_text = &paragraph[start..end];
                let boundary_symbol = if end > 0 && end <= paragraph.len() {
                    // Trim trailing whitespace before looking for the boundary symbol.
                    // This fixes the issue where boundary symbols are not detected when
                    // followed by whitespace (e.g., "Hello. " should detect "." as symbol).
                    let sentence_slice = &paragraph[..end];
                    let trimmed_slice = sentence_slice.trim_end();

                    // Use char_indices for more efficient character iteration on the trimmed slice
                    trimmed_slice
                        .char_indices()
                        .next_back()
                        .and_then(|(idx, _)| {
                            // Extract the last character from the trimmed slice
                            let char_str = &trimmed_slice[idx..];
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

                let start_index = if start == prev_end_byte {
                    prev_end_index
                } else {
                    let safe_prev = paragraph.floor_char_boundary(prev_end_byte);
                    let safe_start = paragraph.floor_char_boundary(start);
                    prev_end_index + paragraph[safe_prev..safe_start].chars().count()
                };
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

                prev_end_index = end_index;
                prev_end_byte = end;
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
        if self.continue_in_next_word(word.trim()) || CONTINUE_AFTER_NONWORD_REGEX.is_match(word) {
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

    /// If `boundary` sits at an orphan trailing quote closer (e.g. `.'` with no
    /// matching opener captured by `QUOTES_REGEX`), advance past the closer, any
    /// trailing whitespace, and any stranded terminator that would otherwise
    /// form a single-punctuation sentence. Returns `boundary` unchanged otherwise.
    fn extend_past_orphan_closer(
        &self,
        paragraph: &str,
        boundary: usize,
        skippable_ranges: &[SkippableRange],
    ) -> usize {
        // If the next char opens a known quoted range, that quote belongs to
        // the upcoming sentence — leave the boundary alone.
        if skippable_ranges.iter().any(|r| r.start == boundary) {
            return boundary;
        }

        // Find an orphan closer starting at `boundary`, if any. Longest first
        // so `''` wins over `'` when both could match.
        let Some(closer) = QUOTE_CLOSERS_BY_LEN.iter().find(|c| {
            paragraph[boundary..].starts_with(**c)
                && is_orphan_closer(paragraph, boundary, c, skippable_ranges)
        }) else {
            return boundary;
        };

        // A symmetric closer (`''`, `'`, `"`, …) that follows a terminator+space
        // and precedes whitespace + a capitalized word is more plausibly the
        // *opener* of the next sentence than a trailing orphan — but only when
        // the same token has already been used as a paired opener/closer
        // earlier in the paragraph. That paired use is the signal that the
        // text is using this token in opener position too; without it, the
        // token is more likely a stray closer (e.g. `… do ? ''` with no
        // earlier opener).
        if is_symmetric_quote_closer(closer) {
            let has_earlier_symmetric_pair = skippable_ranges.iter().any(|r| {
                r.end <= boundary
                    && is_symmetric_quote_range(paragraph, r)
                    && paragraph[r.start..].starts_with(*closer)
            });

            if has_earlier_symmetric_pair {
                let after = &paragraph[boundary + closer.len()..];
                let mut chars = after.chars();
                let first = chars.next();
                if first.is_some_and(char::is_whitespace) {
                    let next_non_ws = chars.find(|c| !c.is_whitespace());
                    if next_non_ws.is_some_and(|c| c.is_ascii_uppercase()) {
                        return boundary;
                    }
                }
            }
        }

        let advance_past_space = |pos: usize| {
            SPACE_AFTER_SEPARATOR
                .find(&paragraph[pos..])
                .map_or(pos, |m| pos + m.end())
        };

        let mut boundary = advance_past_space(boundary + closer.len());
        let sentence_break_regex = self.get_sentence_break_regex();

        // Absorb any stranded terminators (e.g. `'' .`) that would otherwise
        // form a single-punctuation sentence.
        while let Some(m) = sentence_break_regex
            .find(&paragraph[boundary..])
            .filter(|m| m.start() == 0)
        {
            boundary = advance_past_space(boundary + m.end());
        }

        boundary
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
        // Trim trailing whitespace so a stray space before the terminator
        // (`U.S .`) doesn't blank out the last word. `/` joins route names
        // to abbreviations (`171/U.S`) without being a real word boundary,
        // so split on it too. Walk back from the end (rfind) rather than
        // splitting from the start: this is on the per-match hot path and
        // we only need the trailing word.
        let trimmed = text.trim_end();
        match trimmed
            .char_indices()
            .rfind(|(_, c)| c.is_whitespace() || *c == '.' || *c == '/')
        {
            Some((i, c)) => &trimmed[i + c.len_utf8()..],
            None => trimmed,
        }
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

    /// True when the terminator at `[start, end)` looks like a confident
    /// sentence end: a single `.` whose preceding word is a symmetric quote
    /// closer (`''`, `"`, …) and whose follower starts with a capital letter
    /// — the `closer + . + UpperWord` shape. Used as a structural escape
    /// valve for symmetric-pair quote ranges (`''…''`, `'…'`, `"…"`) that the
    /// non-greedy `QUOTES_REGEX` may have mispaired across a real boundary.
    ///
    /// TODO: a starter-word fallback (last_word non-closer + next word in
    /// language's starter list) belongs here once starter-word infrastructure
    /// lands on this branch. Until then, only the closer-branch is implemented.
    fn has_strong_sentence_break(&self, paragraph: &str, start: usize, end: usize) -> bool {
        if &paragraph[start..end] != "." {
            return false;
        }
        let head = &paragraph[..start];

        // The terminator is typically space-separated from the closer
        // (`'' .`), so trim trailing whitespace before walking back to find
        // the last token.
        let trimmed = head.trim_end();

        let last_word = match trimmed
            .char_indices()
            .rfind(|(_, c)| c.is_whitespace() || *c == '.')
        {
            Some((i, c)) => &trimmed[i + c.len_utf8()..],
            None => trimmed,
        };

        if last_word.is_empty() {
            return false;
        }

        if self.is_abbreviation(head, last_word, ".") {
            return false;
        }

        if !is_symmetric_quote_closer(last_word) {
            return false;
        }

        let next_index = paragraph.ceil_char_boundary(start + 1);
        let next_word_approx = self.get_next_word_approx(paragraph, next_index);

        next_word_approx
            .trim_start()
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase())
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
        let matched = &text[start..end];

        // Any coalesced multi-char terminator run (`...`, `!?`, `. . .`, `! ?`)
        // allows leading whitespace before the lowercase continuation test, so
        // `! ? is` and `... no` read as mid-sentence. `chars().nth(1)` rules out
        // a single multi-byte terminator (`。`, `…`), which would otherwise look
        // multi-char by byte length.
        let is_multi_char_run = matched.chars().nth(1).is_some();

        // For any multi-char match (e.g. `...`, `!?`, `!...`), scan continuation
        // and trailing-space extension from the end of the run, not from one byte
        // past its first char.
        let next_index = if is_multi_char_run {
            end
        } else {
            text.ceil_char_boundary(start + 1)
        };

        let next_word_approx = self.get_next_word_approx(text, next_index);

        if let Some(number_ref_match) =
            crate::constants::NUMBERED_REFERENCE_REGEX.find(next_word_approx)
        {
            return Some(next_index + number_ref_match.end());
        }

        let continues = if is_multi_char_run {
            ELLIPSIS_CONTINUE_REGEX.is_match(next_word_approx)
                || (head.chars().next_back().is_some_and(|c| !c.is_whitespace())
                    && ELLIPSIS_GLUED_CONTINUE_REGEX.is_match(next_word_approx))
        } else {
            self.continue_in_next_word(next_word_approx)
        };

        if continues {
            return None;
        }

        // Digit immediately before the period and a digit-bearing alphanumeric
        // token immediately after (no space) is a code-like numbered token,
        // not a sentence end: chess moves (`7.Bg5`). Requiring a digit in the
        // follower keeps quantities like `1,000.That` on the normal boundary
        // path. The lowercase variant (`7.f4`) already passes through
        // `continues`; this handles the uppercase case.
        if matched == "."
            && head.bytes().next_back().is_some_and(|b| b.is_ascii_digit())
            && next_word_approx
                .chars()
                .next()
                .is_some_and(|c| c.is_alphabetic())
            && next_word_approx.bytes().any(|b| b.is_ascii_digit())
        {
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
        if CONTINUE_REGEX.is_match(text_after_boundary) {
            return true;
        }

        // A comma following the terminator (after optional spaces) signals that
        // the period is stray punctuation and the real clause continues. Treat
        // it as a continuation rather than a boundary.
        let trimmed = text_after_boundary.trim_start();
        trimmed.as_bytes().first() == Some(&b',')
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
