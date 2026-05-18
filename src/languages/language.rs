use regex::Regex;
use rustc_hash::FxHashSet;
use std::sync::LazyLock;

use crate::SentenceBoundary;
use crate::constants::EMAIL_REGEX;
use crate::constants::EXCLAMATION_WORDS;
use crate::constants::GLOBAL_SENTENCE_TERMINATORS;
use crate::constants::PARENS_REGEX;
use crate::constants::QUOTE_CLOSERS_BY_LEN;
use crate::constants::QUOTE_PAIRS;
use crate::constants::QUOTES_REGEX;
use crate::constants::QuotePair;
use crate::constants::SPACE_AFTER_SEPARATOR;
use crate::constants::is_sentence_terminator;

use super::trailing_markers::{MarkerTable, classify_trailing_marker, marker_bypasses_suppression};

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
        GLOBAL_SENTENCE_TERMINATORS.iter().collect::<String>()
    );

    Regex::new(&pattern).unwrap()
});

// Matches a lowercase letter or digit, optionally preceded by non-word characters
// (e.g. a space or punctuation). Used by languages that extend the base continuation
// check with their own month lists.
static CONTINUE_AFTER_NONWORD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\W*[0-9a-z]").unwrap());

// Ellipsis continuation: treat a multi-char terminator run as mid-sentence when the follow-up is
// whitespace + a lowercase letter or digit (`... no`, `. . . what`). Languages with a
// capitalized word that is ambiguous with a sentence start (English standalone `I`) extend
// this via the `is_ellipsis_continuation` trait method.
pub(crate) static ELLIPSIS_CONTINUE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+[0-9a-z]").unwrap());

/// Equivalent to regex `^[0-9a-z]`. Direct byte check is faster for simple pattern.
fn starts_with_ascii_lowercase_or_digit(s: &str) -> bool {
    s.as_bytes()
        .first()
        .is_some_and(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9'))
}

/// If the bytes at `at` start a `\n[\r]*\n` paragraph separator, return
/// the byte range of the entire separator, else `None`.
fn paragraph_break_at(bytes: &[u8], at: usize) -> Option<(usize, usize)> {
    if bytes.get(at) != Some(&b'\n') {
        return None;
    }

    let mut end = at + 1;
    while bytes.get(end) == Some(&b'\r') {
        end += 1;
    }

    (bytes.get(end) == Some(&b'\n')).then_some((at, end + 1))
}

/// Replaces the previous paragraph split regex `\n[\r]*\n` with memchr scan for performance.
/// Iterate over `\n[\r]*\n` paragraph separators in `text` as `(start, end)` byte ranges.
pub(crate) fn paragraph_breaks(text: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    let bytes = text.as_bytes();
    let mut cursor = 0;

    std::iter::from_fn(move || {
        loop {
            let newline = cursor + memchr::memchr(b'\n', &bytes[cursor..])?;
            if let Some((start, end)) = paragraph_break_at(bytes, newline) {
                cursor = end;
                return Some((start, end));
            }

            // Lone `\n` — advance past it and keep scanning.
            cursor = newline + 1;
        }
    })
}

/// True iff it finds a `.[ \t]+[A-Z]` shape.
/// Using memchr here instead of regex for speed
fn has_possible_inline_sentence_break(span: &str) -> bool {
    let bytes = span.as_bytes();
    for dot_pos in memchr::memchr_iter(b'.', bytes) {
        let tail = &bytes[dot_pos + 1..];

        let Some(blanks) = tail.iter().position(|&b| !matches!(b, b' ' | b'\t')) else {
            continue;
        };

        if blanks > 0 && tail[blanks].is_ascii_uppercase() {
            return true;
        }
    }

    false
}

fn is_single_ascii_upper(s: &str) -> bool {
    s.len() == 1 && s.as_bytes()[0].is_ascii_uppercase()
}

pub(crate) fn abbreviation_set_contains(set: &FxHashSet<String>, word: &str) -> bool {
    if word.bytes().all(|b| b < 128 && !b.is_ascii_uppercase()) {
        return set.contains(word);
    }

    // For small words a stack buffer avoids a string allocation
    if word.is_ascii() && word.len() <= 32 {
        let mut buf = [0u8; 32];
        for (i, b) in word.bytes().enumerate() {
            buf[i] = b.to_ascii_lowercase();
        }

        // SAFETY: ASCII is valid UTF-8
        let lower = unsafe { std::str::from_utf8_unchecked(&buf[..word.len()]) };
        return set.contains(lower);
    }

    set.contains(word.to_lowercase().as_str())
}

/// True iff `s` (after leading whitespace) begins with a name-initial token:
/// a single uppercase ASCII letter, a `.`, then end-of-string or whitespace.
/// `J. R. Tolkien` triggers. `Jones`, `J.R.R.`, and `A.B` do not.
fn starts_with_initial(s: &str) -> bool {
    let mut chars = s.trim_start().chars();
    let Some(first) = chars.next() else {
        return false;
    };

    first.is_ascii_uppercase()
        && chars.next() == Some('.')
        && chars.next().is_none_or(char::is_whitespace)
}

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
/// inspect the closer's distribution across the paragraph, ignoring
/// occurrences already consumed by a paired quote range and contractions
/// like `wasn't`.
fn is_orphan_closer(
    paragraph: &str,
    boundary: usize,
    closer: &str,
    skippable_ranges: &[SkippableRange],
) -> bool {
    if !is_symmetric_quote_closer(closer) {
        return true;
    }

    let (mut before, mut at_or_after) = (0usize, 0usize);
    for (idx, _) in paragraph.match_indices(closer) {
        if !is_quote_candidate(paragraph, idx, closer, skippable_ranges) {
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

/// Find the first `QUOTE_PAIRS` entry whose `open` is a prefix of `span`.
fn identify_quote_pair(span: &str) -> Option<&'static QuotePair> {
    QUOTE_PAIRS.iter().find(|p| span.starts_with(p.open))
}

/// True when `range` was constructed for a symmetric-pair quote token,
/// e.g., `''…''`, `'…'`, `"…"`.
fn is_symmetric_quote_range(range: &SkippableRange) -> bool {
    range.quote_pair.is_some_and(|p| p.open == p.close)
}

/// True when the paragraph contains an odd number of the symmetric quote
/// token that opens `range`. An odd count guarantees at least one orphan
/// occurrence — and when that orphan sits earlier than a real downstream
/// opener, `QUOTES_REGEX` will mispair across a real sentence break. Even
/// counts are structurally consistent and should be trusted.
fn symmetric_token_count_is_odd(paragraph: &str, range: &SkippableRange) -> bool {
    let Some(pair) = range.quote_pair else {
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

/// Trim leading whitespace from `s`, then if a symmetric quote closer
/// follows, peel one such closer plus its trailing whitespace. Used to
/// look "through" a stray closing apostrophe when checking for a comma
/// continuation, e.g., `. ' , Tim …`.
fn peel_leading_symmetric_quote(s: &str) -> &str {
    let trimmed = s.trim_start();
    let Some(&first) = trimmed.as_bytes().first() else {
        return trimmed;
    };

    // Fast check for ASCII quotes
    if first.is_ascii() && !matches!(first, b'\'' | b'"' | b'`') {
        return trimmed;
    }

    QUOTE_PAIRS
        .iter()
        .filter(|p| p.open == p.close)
        .find_map(|p| trimmed.strip_prefix(p.close))
        .map(str::trim_start)
        .unwrap_or(trimmed)
}

/// True when `idx` lies inside an existing quote `SkippableRange`.
/// Ranges are sorted so use a binary search to filter out irrelevant ranges.
fn is_in_quote_range(ranges: &[SkippableRange], idx: usize) -> bool {
    let pos = ranges.partition_point(|r| r.start <= idx);
    ranges[..pos].iter().any(|r| r.is_quote() && idx < r.end)
}

/// Chars immediately before `idx` and immediately after `idx + token.len()`.
/// Returned as `(prev, next)`; either side is `None` at the text boundary.
fn neighbors(text: &str, idx: usize, token: &str) -> (Option<char>, Option<char>) {
    (
        text[..idx].chars().next_back(),
        text[idx + token.len()..].chars().next(),
    )
}

/// True when the `token` occurrence at `idx` is a contraction (`wasn't`,
/// `o'clock`) sandwiched between two alphanumerics. Such `'` are not quote
/// characters. Counting them flips the parity-based orphan check.
fn is_contraction_quote(text: &str, idx: usize, token: &str) -> bool {
    let (prev, next) = neighbors(text, idx, token);
    prev.is_some_and(|c| c.is_alphanumeric()) && next.is_some_and(|c| c.is_alphanumeric())
}

/// True when the `token` occurrence at `idx` is a free-standing quote
/// candidate not already inside a paired quote range and not a contraction.
fn is_quote_candidate(text: &str, idx: usize, token: &str, ranges: &[SkippableRange]) -> bool {
    !is_in_quote_range(ranges, idx) && !is_contraction_quote(text, idx, token)
}

/// True when the `'` or `` ` `` at `idx` is preceded by start of text or
/// whitespace AND followed by whitespace. The shape `QUOTES_REGEX`'s guarded
/// pattern can't pair. It requires `\b` after the opener.
fn is_opener_shape(text: &str, idx: usize, token: &str) -> bool {
    let (prev, next) = neighbors(text, idx, token);
    prev.is_none_or(char::is_whitespace) && next.is_some_and(char::is_whitespace)
}

/// True when `text[from..]` starts with one or more ASCII blanks followed by
/// an ASCII uppercase letter, the signature of a fresh utterance opening.
fn starts_new_utterance(text: &str, from: usize) -> bool {
    let tail = &text[from..];
    let trimmed = tail.trim_start_matches([' ', '\t']);

    trimmed.len() < tail.len()
        && trimmed
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_uppercase)
}

/// True when the candidate `'` or `` ` `` at `opener` should pair with the
/// candidate at `closer` into a quote range. Rejects two shapes:
/// * `opener` isn't space-padded. It's not opener-like to begin with.
/// * The span looks like two back to back utterances rather than one
///   multi-sentence quotation. The closer itself starts a fresh utterance
///   (whitespace + uppercase) AND the intervening text contains an inline
///   `[.!] + ws + uppercase` sentence break.
fn quote_candidates_should_pair(text: &str, opener: usize, closer: usize, token: &str) -> bool {
    if !is_opener_shape(text, opener, token) {
        return false;
    }

    let span = &text[opener + token.len()..closer];
    let after_closer = closer + token.len();
    let back_to_back =
        starts_new_utterance(text, after_closer) && has_possible_inline_sentence_break(span);

    !back_to_back
}

/// Define quote mispairings to improve readability in the orphan quote handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuoteMispairing {
    #[default]
    None,
    Certain,
    Possible,
}

/// Tag each symmetric-pair quote range with a mispairing label.
fn populate_quote_mispairing(paragraph: &str, ranges: &mut [SkippableRange]) {
    for i in 0..ranges.len() {
        if !ranges[i].is_quote() {
            continue;
        }

        let range = ranges[i];

        let class = if !is_symmetric_quote_range(&range) {
            QuoteMispairing::None
        } else if quote_partially_overlaps_parens(&range, ranges) {
            QuoteMispairing::Certain
        } else if symmetric_token_count_is_odd(paragraph, &range) {
            QuoteMispairing::Possible
        } else {
            QuoteMispairing::None
        };

        ranges[i].quote_mispairing = class;
    }
}

/// Append `'…'` / `` `…` `` ranges that `QUOTES_REGEX` couldn't pair.
/// Guarded patterns require `\b` immediately after the opener, so
/// space padded openers like `' word ` go unpaired even when they form a
/// real `' … '` pair.
fn append_space_padded_quote_pairs(text: &str, ranges: &mut Vec<SkippableRange>) {
    let bytes = text.as_bytes();

    // Fast check
    if memchr::memchr2(b'\'', b'`', bytes).is_none() {
        return;
    }

    for pair in QUOTE_PAIRS
        .iter()
        .filter(|p| p.ambiguous && p.open == p.close)
    {
        let token = pair.close;
        debug_assert_eq!(
            token.len(),
            1,
            "ambiguous symmetric tokens are single-byte ASCII"
        );

        if !bytes.contains(&token.as_bytes()[0]) {
            continue;
        }

        let mut pending: Option<usize> = None;
        for (idx, _) in text.match_indices(token) {
            if !is_quote_candidate(text, idx, token, ranges) {
                continue;
            }

            if let Some(opener) = pending
                && quote_candidates_should_pair(text, opener, idx, token)
            {
                let end = idx + token.len();
                ranges.push(SkippableRange::new_quote(opener, end, pair));
                pending = None;
            } else {
                pending = Some(idx);
            }
        }
    }
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
fn find_terminator_matches(text: &str, regex: &Regex, out: &mut Vec<(usize, usize)>) {
    out.clear();

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
    pub quote_pair: Option<&'static QuotePair>,
    pub quote_mispairing: QuoteMispairing,
}

impl SkippableRange {
    pub fn new(start: usize, end: usize, range_type: SkippableRangeType) -> Self {
        Self {
            start,
            end,
            range_type,
            quote_pair: None,
            quote_mispairing: QuoteMispairing::None,
        }
    }

    pub fn new_quote(start: usize, end: usize, pair: &'static QuotePair) -> Self {
        Self {
            start,
            end,
            range_type: SkippableRangeType::Quote,
            quote_pair: Some(pair),
            quote_mispairing: QuoteMispairing::None,
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
        // CRITICAL: We track both byte offsets AND character offsets separately.
        // This is essential for correct handling of multi-byte UTF-8 characters like CJK and emojis.
        // Example: "日本語" is 3 characters but 9 bytes:
        //   - byte offset: 0..9
        //   - char offset: 0..3
        // Roughly estimate 4 sentences per paragraph
        let estimated_paragraphs = (estimated_sentences / 4).max(1);
        let mut paragraphs: Vec<&str> = Vec::with_capacity(estimated_paragraphs);
        let mut paragraph_offsets: Vec<usize> = Vec::with_capacity(estimated_paragraphs);
        let mut paragraph_char_offsets: Vec<usize> = Vec::with_capacity(estimated_paragraphs);

        let mut last_end = 0;
        let mut current_char_offset = 0;
        for (sep_start, sep_end) in paragraph_breaks(text) {
            let paragraph = &text[last_end..sep_start];
            paragraphs.push(paragraph);
            paragraph_offsets.push(last_end);
            paragraph_char_offsets.push(current_char_offset);

            let separator_chars = text[sep_start..sep_end].chars().count();
            current_char_offset += paragraph.chars().count() + separator_chars;
            last_end = sep_end;
        }

        // Final paragraph after the last separator (the whole text if none).
        paragraphs.push(&text[last_end..]);
        paragraph_offsets.push(last_end);
        paragraph_char_offsets.push(current_char_offset);

        // Pre-allocate sentence_boundaries once and reuse for all paragraphs
        let estimated_paragraph_sentences = 10; // reasonable default for typical paragraphs
        let mut sentence_boundaries = Vec::with_capacity(estimated_paragraph_sentences);
        let mut matches: Vec<(usize, usize)> = Vec::with_capacity(estimated_paragraph_sentences);
        let sentence_break_regex = self.get_sentence_break_regex();

        for (pindex, paragraph) in paragraphs.iter().enumerate() {
            if pindex > 0 {
                let paragraph_start = paragraph_offsets[pindex];
                let separator_start = paragraph_offsets[pindex - 1] + paragraphs[pindex - 1].len();
                let separator = &text[separator_start..paragraph_start];
                let paragraph_char_start = paragraph_char_offsets[pindex];

                boundaries.push(SentenceBoundary {
                    start_index: paragraph_char_start - separator.chars().count(),
                    end_index: paragraph_char_start,
                    start_byte: paragraph_start - separator.len(),
                    end_byte: paragraph_start,
                    text: separator,
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

            find_terminator_matches(paragraph, sentence_break_regex, &mut matches);
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

            'next_match: for &(start, end) in &matches {
                let Some(mut boundary) = self.find_boundary(paragraph, start, end) else {
                    continue;
                };

                for range in &skippable_ranges {
                    if !range.contains(boundary) {
                        continue;
                    }

                    if self.is_symmetric_quote_mispairing(paragraph, range, start, end) {
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
                        .and_then(|(idx, ch)| {
                            if is_sentence_terminator(ch) {
                                Some(&trimmed_slice[idx..])
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
    /// Returns an empty set by default.
    fn get_abbreviations(&self) -> &FxHashSet<String> {
        static EMPTY_ABBREVS: LazyLock<FxHashSet<String>> = LazyLock::new(FxHashSet::default);
        &EMPTY_ABBREVS
    }

    /// Returns a set of safe sentence-opener words for this language.
    /// Restrict the list to function words and auxiliaries that almost never appear
    /// capitalized mid-sentence. Never include proper nouns.
    /// Returns an empty set to preserve default behaviour. Languages opt in by overriding.
    fn get_sentence_starters(&self) -> &FxHashSet<String> {
        static EMPTY_STARTERS: LazyLock<FxHashSet<String>> = LazyLock::new(FxHashSet::default);
        &EMPTY_STARTERS
    }

    /// Words permitted in fronted adverbial phrases. Used in `prefix_is_purely_fronting`.
    /// Languages must opt in.
    /// Returns an empty set by default.
    fn get_fronting_words(&self) -> &FxHashSet<String> {
        static EMPTY_FRONTING: LazyLock<FxHashSet<String>> = LazyLock::new(FxHashSet::default);
        &EMPTY_FRONTING
    }

    /// Trailing-marker lookup table.
    /// Languages must opt in.
    /// Returns an empty MarkerTable by default.
    #[inline]
    fn get_trailing_markers(&self) -> &'static MarkerTable {
        MarkerTable::empty()
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
            if ch.is_whitespace() || is_sentence_terminator(ch) {
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
                    && is_symmetric_quote_range(r)
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
        let last_word = self.get_last_word(head);
        self.is_abbreviation_for(last_word, separator)
    }

    /// Same check as `is_abbreviation` but skips the `get_last_word(head)` call
    /// when the caller already has the trailing word. Used on the hot path in
    /// `find_boundary`, which computes `last_word` once and shares it with
    /// `is_name_initial`/`next_word_is_sentence_starter`.
    fn is_abbreviation_for(&self, last_word: &str, separator: &str) -> bool {
        if self.get_abbreviation_char() != separator || last_word.is_empty() {
            return false;
        }
        abbreviation_set_contains(self.get_abbreviations(), last_word)
    }

    /// Detects a name initial: a single uppercase ASCII letter followed by a
    /// period in a position that looks like part of a name. Returns true when
    /// the immediately preceding token in `head` starts with an uppercase
    /// ASCII letter (`Albert I.`, `George W.`) or the immediately following
    /// token is itself an initial (`J. R. R. Tolkien`, including the
    /// sentence-initial position where there is no preceding token).
    ///
    /// Conservative: ASCII-only on both sides, so non-Latin scripts are
    /// unaffected. Caller is expected to gate this on the matched terminator
    /// being a single `.` and on `last_word` being a single uppercase letter.
    /// The helper re-checks the latter so it is safe to call standalone.
    fn is_name_initial(&self, head: &str, next_word_approx: &str) -> bool {
        let last_word = self.get_last_word(head);
        self.is_name_initial_for(head, last_word, next_word_approx)
    }

    /// Same check as `is_name_initial` but skips the `get_last_word(head)` call
    /// when the caller already has the trailing word. Used on the hot path in
    /// `find_boundary`.
    fn is_name_initial_for(&self, head: &str, last_word: &str, next_word_approx: &str) -> bool {
        if !is_single_ascii_upper(last_word) {
            return false;
        }

        // Preceding-token rule: trim the initial and any separators
        // get_last_word splits on (whitespace, `.`, `/`), then take the
        // trailing word of what's left.
        let prefix = head[..head.len() - last_word.len()]
            .trim_end_matches(|c: char| c.is_whitespace() || c == '.' || c == '/');

        if self
            .get_last_word(prefix)
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase())
        {
            return true;
        }

        starts_with_initial(next_word_approx)
    }

    /// Returns true if the next non-space token in `next_word_approx` is a known
    /// sentence opener for this language. Overrides sentence break suppression of
    /// abbreviation or name-initial paths. A listed starter word strongly signals
    /// the start of a new sentence.
    fn next_word_is_sentence_starter(&self, next_word_approx: &str) -> bool {
        let starters = self.get_sentence_starters();
        if starters.is_empty() {
            return false;
        }

        let trimmed = next_word_approx.trim_start();

        let word_end = trimmed
            .find(|c: char| c.is_whitespace() || c == ',' || is_sentence_terminator(c))
            .unwrap_or(trimmed.len());

        if word_end == 0 {
            return false;
        }

        let starter_candidate = &trimmed[..word_end];
        starters.contains(starter_candidate)
    }

    /// One way override that lets `find_boundary` keep a boundary the abbreviation / name-initial path would
    /// otherwise suppress. Fires when the next word is a registered sentence starter and the trailing token:
    /// - Starts with an uppercase letter: initials (`I.`), names (`Penn.`), acronyms (`BART.`).
    /// - A known multi dot abbreviation (`w.e.f.`).
    /// - A multi character lowercase abbreviation (`etc.`, `man.`).
    fn should_override_abbrev_suppression_for(
        &self,
        head: &str,
        last_word: &str,
        next_is_starter: bool,
    ) -> bool {
        if !next_is_starter {
            return false;
        }

        let tail_starts_uppercase = last_word
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase());

        if tail_starts_uppercase {
            return true;
        }

        if self.is_multi_dot_abbreviation(head, last_word.len()) {
            return true;
        }

        last_word.chars().nth(1).is_some() && self.get_abbreviations().contains(last_word)
    }

    /// True when `head`'s trailing token is a multi-dot abbreviation listed
    /// in this language's abbreviation table (`w.e.f`, `U.S.`, ...).
    fn is_multi_dot_abbreviation(&self, head: &str, tail_len: usize) -> bool {
        let last_word_full = self.get_last_word_full(head);
        if last_word_full.len() <= tail_len {
            return false;
        }

        abbreviation_set_contains(self.get_abbreviations(), last_word_full)
    }

    /// Like `get_last_word`, but keeps internal `.`s so multi-dot
    /// abbreviations (`w.e.f`, `U.S`, `p.m`) are returned whole. Splits only
    /// on whitespace and `/`. Used by abbreviation lookup so the full token
    /// can be matched against the abbreviation table.
    fn get_last_word_full<'a>(&self, text: &'a str) -> &'a str {
        text.trim_end()
            .rsplit(|c: char| c.is_whitespace() || c == '/')
            .next()
            .expect("str::rsplit always yields at least one element")
    }

    /// Extracts the last word from the given text by splitting on whitespace, periods, and slashes.
    /// Used primarily by abbreviation detection to check if the word before a potential
    /// sentence boundary is a known abbreviation. Returns an empty string if no words
    /// are found. This is a performance-optimized version that avoids collecting all words.
    fn get_last_word<'a>(&self, text: &'a str) -> &'a str {
        // Trim trailing whitespace so a stray space before the terminator
        // (`U.S .`) doesn't blank out the last word. `/` joins route names
        // to abbreviations (`171/U.S`) without being a real word boundary,
        // so split on it too.
        text.trim_end()
            .rsplit(|c: char| c.is_whitespace() || c == '.' || c == '/')
            .next()
            .expect("str::rsplit always yields at least one element")
    }

    /// Checks if a potential sentence boundary is actually an exclamation word that shouldn't
    /// trigger a sentence break. Examines the last word before the boundary and checks if
    /// it's in the list of known exclamation words (like "Hey!" or "Wow!").
    /// Returns true if this is an exclamation that should not break the sentence.
    fn is_exclamation(&self, head: &str, _tail: &str) -> bool {
        let last_word = self.get_last_word(head);
        self.is_exclamation_for(last_word)
    }

    /// Same check as `is_exclamation` but skips the `get_last_word(head)`
    /// call when the caller already has the trailing word. Used on the hot
    /// path in `find_boundary`.
    fn is_exclamation_for(&self, last_word: &str) -> bool {
        if last_word.is_empty() {
            return false;
        }

        EXCLAMATION_WORDS
            .iter()
            .any(|w| w.strip_suffix('!').is_some_and(|p| p == last_word))
    }

    /// True when this symmetric-pair quote range (`''…''`, `'…'`, `"…"`) is
    /// a likely `QUOTES_REGEX` mispairing across a sentence break.
    /// - Certain: range straddles a parens boundary. Override always
    ///   fires. A quote pair shouldn't cross a parens edge.
    /// - Possible: paragraph has an odd token count (one or more
    ///   orphans). Override fires only if `[start, end)` also looks like
    ///   a strong sentence break.
    /// - None: not a symmetric pair or evenly balanced. No override.
    fn is_symmetric_quote_mispairing(
        &self,
        paragraph: &str,
        range: &SkippableRange,
        start: usize,
        end: usize,
    ) -> bool {
        match range.quote_mispairing {
            QuoteMispairing::None => false,
            QuoteMispairing::Certain => true,
            QuoteMispairing::Possible => self.has_strong_sentence_break(paragraph, start, end),
        }
    }

    /// True when the terminator at `[start, end)` looks like a confident
    /// sentence end: a single `.` whose preceding word is a symmetric quote
    /// closer (`''`, `"`, …) and whose follower starts with a capital letter
    /// — the `closer + . + UpperWord` shape. Used as a structural escape
    /// valve for symmetric-pair quote ranges (`''…''`, `'…'`, `"…"`) that the
    /// non-greedy `QUOTES_REGEX` may have mispaired across a real boundary.
    fn has_strong_sentence_break(&self, paragraph: &str, start: usize, end: usize) -> bool {
        if end - start != 1 || paragraph.as_bytes()[start] != b'.' {
            return false;
        }

        debug_assert!(paragraph.is_char_boundary(start + 1));
        let next_word_approx = self.get_next_word_approx(paragraph, start + 1);
        let trimmed_next = next_word_approx.trim_start();
        if !trimmed_next
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase())
        {
            return false;
        }

        let head = &paragraph[..start];
        let last_word = self.get_last_word(head);
        if last_word.is_empty() || is_single_ascii_upper(last_word) {
            return false;
        }

        if self.is_abbreviation(head, last_word, ".")
            || self.is_multi_dot_abbreviation(head, last_word.len())
        {
            return false;
        }

        if is_symmetric_quote_closer(last_word) {
            return true;
        }

        self.next_word_is_sentence_starter(trimmed_next)
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

        // Scan continuation and trailing-space extension from the end of the
        // matched terminator. For a single-char match `end` is one-past the
        // terminator char; for a coalesced run it's the end of the whole run.
        let next_index = end;

        let next_word_approx = self.get_next_word_approx(text, next_index);

        if let Some(number_ref_match) =
            crate::constants::NUMBERED_REFERENCE_REGEX.find(next_word_approx)
        {
            return Some(next_index + number_ref_match.end());
        }

        // Any coalesced multi-char terminator run (`...`, `!?`, `. . .`, `! ?`)
        // allows leading whitespace before the lowercase continuation test, so
        // `! ? is` and `... no` read as mid-sentence. `chars().nth(1)` rules out
        // a single multi-byte terminator (`。`, `…`), which would otherwise look
        // multi-char by byte length.
        let is_multi_char_run = matched.chars().nth(1).is_some();

        let continues = if is_multi_char_run {
            self.is_ellipsis_continuation(next_word_approx)
                || (head.chars().next_back().is_some_and(|c| !c.is_whitespace())
                    && starts_with_ascii_lowercase_or_digit(next_word_approx))
        } else {
            self.continue_in_next_word(next_word_approx)
                // e.g., "Father Came Too ! is a British comedy film".
                || (matches!(matched, "!" | "?")
                    && matches!(head.as_bytes().last(), Some(b' ' | b'\t'))
                    && CONTINUE_AFTER_NONWORD_REGEX.is_match(next_word_approx))
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

        let last_word = self.get_last_word(head);

        if matched == "." {
            // Name-initial detection and the abbreviation table both feed
            // one suppression flag, which `should_override_abbrev_suppression_for`
            // can lift when the next token is a sentence starter.
            let is_initial_letter = is_single_ascii_upper(last_word);

            let name_initial_abbreviation_suppress = (is_initial_letter
                && self.is_name_initial_for(head, last_word, next_word_approx))
                || self.is_abbreviation_for(last_word, ".");

            let marker = classify_trailing_marker(head, self.get_trailing_markers());

            // A `.` is suppressed by default when its trailing token is a
            // known abbreviation, name-initial, OR a known marker. The marker
            // bypass and the starter override can each lift the suppression
            // independently.
            if name_initial_abbreviation_suppress || marker.is_some() {
                let next_is_starter = self.next_word_is_sentence_starter(next_word_approx);
                let marker_bypass = marker.as_ref().is_some_and(|m| {
                    marker_bypasses_suppression(m, next_word_approx, next_is_starter, self)
                });

                if !marker_bypass
                    && !self.should_override_abbrev_suppression_for(
                        head,
                        last_word,
                        next_is_starter,
                    )
                {
                    return None;
                }
            }
        } else if self.is_abbreviation_for(last_word, matched) {
            return None;
        }

        if self.is_exclamation_for(last_word) {
            return None;
        }

        if let Some(space_after_sep_match) =
            crate::constants::SPACE_AFTER_SEPARATOR.find(next_word_approx)
        {
            return Some(next_index + space_after_sep_match.end());
        }

        Some(end)
    }

    /// True when text following a multi-char terminator run (`...`, `! ?`,
    /// `. . .`) continues the current sentence rather than starting a new one.
    /// The default treats only whitespace + a lowercase letter/digit as
    /// continuation.
    fn is_ellipsis_continuation(&self, text_after_run: &str) -> bool {
        ELLIPSIS_CONTINUE_REGEX.is_match(text_after_run)
    }

    /// Determines if the text after a potential boundary indicates the sentence should continue.
    /// Returns true if the next word starts with a lowercase letter or number, suggesting
    /// the sentence is continuing rather than starting a new one. This helps avoid breaking
    /// sentences at abbreviations or in the middle of compound sentences.
    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        if starts_with_ascii_lowercase_or_digit(text_after_boundary) {
            return true;
        }

        peel_leading_symmetric_quote(text_after_boundary).starts_with(',')
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
            let pair = identify_quote_pair(&text[mat.start()..])
                .expect("QUOTES_REGEX match must start with a known pair opener");

            skippable_ranges.push(SkippableRange::new_quote(mat.start(), mat.end(), pair));
        }

        append_space_padded_quote_pairs(text, &mut skippable_ranges);

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

        // Cache mispairing on each quote range for re-use
        populate_quote_mispairing(text, &mut skippable_ranges);

        skippable_ranges
    }
}
