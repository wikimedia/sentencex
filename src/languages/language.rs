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

/// True when a `.` sits inside a code-like numbered token rather than ending a
/// sentence. A digit immediately before, and an alphanumeric token with a digit
/// after with no space, e.g. the chess move `7.Bg5`.
fn is_code_like_numbered_token(head: &str, next_word_approx: &str) -> bool {
    head.bytes().next_back().is_some_and(|b| b.is_ascii_digit())
        && next_word_approx.starts_with(|c: char| c.is_alphabetic())
        && next_word_approx.bytes().any(|b| b.is_ascii_digit())
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

/// Single-char quote openers whose `open`/`close` contain no ASCII `'` or backtick.
/// Map to `QuotePair`.
static CLEAN_QUOTE_OPENERS: LazyLock<Vec<(char, &'static QuotePair)>> = LazyLock::new(|| {
    let is_clean = |s: &str| !s.contains(['\'', '`']);

    QUOTE_PAIRS
        .iter()
        .filter(|p| is_clean(p.open) && is_clean(p.close))
        .filter_map(|p| {
            let mut chars = p.open.chars();
            let c = chars.next()?;
            chars.next().is_none().then_some((c, p))
        })
        .collect()
});

/// The clean pair opened by `c`
fn clean_opener_pair(c: char) -> Option<&'static QuotePair> {
    CLEAN_QUOTE_OPENERS
        .iter()
        .find_map(|&(opener, pair)| (opener == c).then_some(pair))
}

/// Fast path when the text has no `'` or backtick and no `0xE3` lead byte (CJK variants).
fn scan_unambiguous_quotes(text: &str, out: &mut Vec<SkippableRange>) {
    let bytes = text.as_bytes();
    let mut cursor = 0;

    while let Some(rel) = memchr::memchr3(b'"', 0xC2, 0xE2, &bytes[cursor..]) {
        let opener = cursor + rel;

        let c = text[opener..]
            .chars()
            .next()
            .expect("a lead byte cannot be at end of text");

        let Some(pair) = clean_opener_pair(c) else {
            cursor = opener + c.len_utf8();
            continue;
        };

        let content_start = opener + pair.open.len();
        match text[content_start..].find(pair.close) {
            Some(off) => {
                let end = content_start + off + pair.close.len();
                out.push(SkippableRange::new_quote(opener, end, pair));
                cursor = end;
            }

            // Opener with no closer
            None => cursor = content_start,
        }
    }
}

/// True when `range` was constructed for a symmetric-pair quote token,
/// e.g., `''…''`, `'…'`, `"…"`.
fn is_symmetric_quote_range(range: &SkippableRange) -> bool {
    range.quote_pair.is_some_and(|p| p.open == p.close)
}

/// Last symmetric quote token's whole paragraph occurrence parity.
#[derive(Default)]
struct ParityCache {
    last: Option<(&'static str, bool)>,
}

impl ParityCache {
    fn token_count_is_odd(&mut self, paragraph: &str, token: &'static str) -> bool {
        if let Some((seen, odd)) = self.last
            && seen == token
        {
            return odd;
        }

        let odd = paragraph.matches(token).count() % 2 == 1;
        self.last = Some((token, odd));

        odd
    }
}

/// True when the paragraph contains an odd number of the symmetric quote
/// token that opens `range`. An odd count guarantees at least one orphan
/// occurrence — and when that orphan sits earlier than a real downstream
/// opener, `QUOTES_REGEX` will mispair across a real sentence break. Even
/// counts are structurally consistent and should be trusted.
fn symmetric_token_count_is_odd(
    paragraph: &str,
    range: &SkippableRange,
    cache: &mut ParityCache,
) -> bool {
    let Some(pair) = range.quote_pair else {
        return false;
    };

    cache.token_count_is_odd(paragraph, pair.open)
}

/// The `parens` are sorted, mutually non-overlapping.
/// Find the one containing byte offset `x` (`p.start < x < p.end`).
fn paren_containing(parens: &[SkippableRange], x: usize) -> Option<&SkippableRange> {
    parens
        .partition_point(|p| p.start < x)
        .checked_sub(1)
        .map(|i| &parens[i])
        .filter(|p| x < p.end)
}

/// True when `quote` partially overlaps any paren in `parens`, i.e., one part in, one part out.
/// Full containment returns false.
/// `parens` must be sorted by start and disjoint.
fn quote_partially_overlaps_parens(quote: &SkippableRange, parens: &[SkippableRange]) -> bool {
    debug_assert!(
        parens.windows(2).all(|w| w[0].end <= w[1].start),
        "parens must be sorted and non-overlapping"
    );

    if let Some(p) = paren_containing(parens, quote.start)
        && p.end < quote.end
    {
        return true;
    }

    if let Some(p) = paren_containing(parens, quote.end)
        && p.start > quote.start
    {
        return true;
    }

    false
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
    let mut cache = ParityCache::default();

    let parens: Vec<SkippableRange> = ranges
        .iter()
        .filter(|r| r.range_type == SkippableRangeType::Parentheses)
        .copied()
        .collect();

    for slot in ranges.iter_mut() {
        if !slot.is_quote() {
            continue;
        }

        let range = *slot;

        let class = if !is_symmetric_quote_range(&range) {
            QuoteMispairing::None
        } else if quote_partially_overlaps_parens(&range, &parens) {
            QuoteMispairing::Certain
        } else if symmetric_token_count_is_odd(paragraph, &range, &mut cache) {
            QuoteMispairing::Possible
        } else {
            QuoteMispairing::None
        };

        slot.quote_mispairing = class;
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

    // Faster path for ASCII + DEFAULT_SENTENCE_BREAK_REGEX
    if std::ptr::eq(regex, &*DEFAULT_SENTENCE_BREAK_REGEX) && text.is_ascii() {
        scan_ascii_matches(text, out);
        return;
    }

    for m in regex.find_iter(text) {
        fold_match(out, text, m.start(), m.end());
    }
}

/// Push `[start, end)` onto `out`. If `should_fold` says the two form a single run
/// like `Happy ! . . .`, fold it into the previous match.
fn fold_match(out: &mut Vec<(usize, usize)>, text: &str, start: usize, end: usize) {
    if let Some(last) = out.last_mut()
        && should_fold(text, *last, start, end)
    {
        last.1 = end;
        return;
    }

    out.push((start, end));
}

/// True when match `[start, end)` should fold onto the previous match `[prev_start, prev_end)`,
/// i.e., whitespace separated dot only run like `. . .` following a `!`/`?`/`…` ending.
fn should_fold(
    text: &str,
    (prev_start, prev_end): (usize, usize),
    start: usize,
    end: usize,
) -> bool {
    let prev = &text[prev_start..prev_end];
    let candidate = &text[start..end];
    let gap = &text[prev_end..start];

    let is_blank = |c: char| matches!(c, ' ' | '\t');
    let prev_is_emphatic = prev.ends_with(['!', '?', '…']);
    let candidate_is_dot_run =
        candidate.starts_with('.') && candidate.chars().all(|c| c == '.' || is_blank(c));
    let separated_by_blanks = !gap.is_empty() && gap.chars().all(is_blank);

    prev_is_emphatic && candidate_is_dot_run && separated_by_blanks
}

/// ASCII fast path for `find_terminator_matches`.
/// Only valid for the default regex.
/// branch 1 `\.(?:[ \t]+\.){2,}` (3+ blank separated dots).
/// branch 2 `[!?](?:[ \t]+[!?])+` (2+ blank separated `!`/`?`)
/// branch 3 (`[.!?]+`) via `contiguous_run`
fn scan_ascii_matches(text: &str, out: &mut Vec<(usize, usize)>) {
    let bytes = text.as_bytes();

    let mut cursor = 0;
    while let Some(rel) = memchr::memchr3(b'.', b'!', b'?', &bytes[cursor..]) {
        let p = cursor + rel;

        let spaced = if bytes[p] == b'.' {
            spaced_run(bytes, p, |b| b == b'.', 3)
        } else {
            spaced_run(bytes, p, |b| b == b'!' || b == b'?', 2)
        };

        let end = spaced.unwrap_or_else(|| contiguous_run(bytes, p));

        fold_match(out, text, p, end);
        cursor = end;
    }
}

/// Scans a space/tab-separated run of `is_member` bytes starting at `p`.
/// Returns the offset just past the last member,
/// or `None` if the run has fewer than `min_total` members.
fn spaced_run(
    bytes: &[u8],
    p: usize,
    is_member: impl Fn(u8) -> bool,
    min_total: usize,
) -> Option<usize> {
    let mut count = 1;
    let mut end = p + 1;

    loop {
        let mut k = end;
        while matches!(bytes.get(k), Some(b' ' | b'\t')) {
            k += 1;
        }

        if k > end && bytes.get(k).is_some_and(|&b| is_member(b)) {
            count += 1;
            end = k + 1;
        } else {
            break;
        }
    }

    (count >= min_total).then_some(end)
}

/// End offset of the contiguous terminator run starting at `p`
fn contiguous_run(bytes: &[u8], p: usize) -> usize {
    let mut end = p + 1;

    while matches!(bytes.get(end), Some(b'.' | b'!' | b'?')) {
        end += 1;
    }

    end
}

/// Scratch buffer
#[derive(Default)]
struct ParagraphScratch {
    sentence_boundaries: Vec<usize>,
    matches: Vec<(usize, usize)>,
    skippable_ranges: Vec<SkippableRange>,
}

impl ParagraphScratch {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            sentence_boundaries: Vec::with_capacity(capacity),
            matches: Vec::with_capacity(capacity),
            skippable_ranges: Vec::with_capacity(capacity),
        }
    }
}

fn boundary_symbol(paragraph: &str, end: usize) -> Option<&str> {
    let trimmed = paragraph[..end].trim_end();
    trimmed
        .char_indices()
        .next_back()
        .and_then(|(idx, ch)| is_sentence_terminator(ch).then(|| &trimmed[idx..]))
}

fn push_separator_boundary<'a>(
    boundaries: &mut Vec<SentenceBoundary<'a>>,
    separator: &'a str,
    start_byte: usize,
    end_byte: usize,
    char_offset: &mut usize,
) {
    let separator_chars = separator.chars().count();

    boundaries.push(SentenceBoundary {
        start_index: *char_offset,
        end_index: *char_offset + separator_chars,
        start_byte,
        end_byte,
        text: separator,
        boundary_symbol: None,
        is_paragraph_break: true,
    });

    *char_offset += separator_chars;
}

/// Convert the paragraph's sentence-break offsets into `SentenceBoundary` values,
/// advancing the running character cursor `char_offset`.
fn push_paragraph_sentences<'a>(
    paragraph: &'a str,
    para_start: usize,
    sentence_boundaries: &[usize],
    char_offset: &mut usize,
    boundaries: &mut Vec<SentenceBoundary<'a>>,
) {
    debug_assert_eq!(sentence_boundaries.first().copied(), Some(0));
    debug_assert_eq!(sentence_boundaries.last().copied(), Some(paragraph.len()));

    for window in sentence_boundaries.windows(2) {
        let seg_start = window[0];
        let seg_end = window[1];
        let sentence_text = &paragraph[seg_start..seg_end];
        let end_offset = *char_offset + sentence_text.chars().count();

        boundaries.push(SentenceBoundary {
            start_index: *char_offset,
            end_index: end_offset,
            start_byte: para_start + seg_start,
            end_byte: para_start + seg_end,
            text: sentence_text,
            boundary_symbol: boundary_symbol(paragraph, seg_end),
            is_paragraph_break: false,
        });

        *char_offset = end_offset;
    }
}

/// Properties of the sorted non list region of a paragraph's skippable ranges.
/// `len` is the number of skippable ranges.
/// `binary_search` is whether to use binary search when searching in the ranges, i.e., is there enough data to
/// justify the overhead of binary search.
#[derive(Clone, Copy)]
struct NonListRegion {
    len: usize,
    binary_search: bool,
}

/// Minimum number of ranges required before it's worth while to use binary search (rather than a linear scan).
const BINARY_SEARCH_MIN_RANGES: usize = 64;

/// Get the byte offsets in `paragraph` where sentences break. Skip terminators inside quotes, parens, and lists.
fn collect_sentence_breaks<L: Language + ?Sized>(
    lang: &L,
    paragraph: &str,
    sentence_break_regex: &Regex,
    scratch: &mut ParagraphScratch,
) {
    let ParagraphScratch {
        sentence_boundaries,
        matches,
        skippable_ranges,
    } = scratch;

    sentence_boundaries.clear();
    sentence_boundaries.push(0);

    find_terminator_matches(paragraph, sentence_break_regex, matches);
    lang.get_skippable_ranges(paragraph, skippable_ranges);

    let non_list_len = skippable_ranges.len();
    let non_list_region = NonListRegion {
        len: non_list_len,
        binary_search: non_list_len > BINARY_SEARCH_MIN_RANGES && !ranges_overlap(skippable_ranges),
    };

    let list_starts = super::list_markers::detect_list_items(paragraph);
    add_list_item_ranges(skippable_ranges, &list_starts, paragraph.len());

    for &(match_start, match_end) in matches.iter() {
        let Some(boundary) = lang.find_boundary(paragraph, match_start, match_end) else {
            continue;
        };

        let break_at = match containing_range(
            lang,
            paragraph,
            boundary,
            match_start,
            match_end,
            skippable_ranges,
            non_list_region,
        ) {
            Some(range) => inner_terminator_boundary(lang, paragraph, range, boundary),
            None => Some(lang.extend_past_orphan_closer(paragraph, boundary, skippable_ranges)),
        };

        if let Some(break_at) = break_at {
            push_if_increasing(sentence_boundaries, break_at);
        }
    }

    merge_list_item_boundaries(sentence_boundaries, &list_starts);

    if *sentence_boundaries.last().unwrap() != paragraph.len() {
        sentence_boundaries.push(paragraph.len());
    }
}

/// True iff a range overlaps another one.
fn ranges_overlap(start_sorted_ranges: &[SkippableRange]) -> bool {
    let mut max_end = 0;

    for r in start_sorted_ranges {
        if r.start < max_end {
            return true;
        }

        max_end = max_end.max(r.end);
    }

    false
}

/// Binary search the non list region of `ranges` for the range satisfying `is_break`.
/// Fall back to the appended list ranges.
/// `ranges` needs to be sorted and non overlapping.
fn select_containing_binary(
    ranges: &[SkippableRange],
    non_list_len: usize,
    boundary: usize,
    is_break: impl Fn(&SkippableRange) -> bool,
) -> Option<&SkippableRange> {
    let (non_list, list) = ranges.split_at(non_list_len);

    non_list
        .partition_point(|r| r.start < boundary)
        .checked_sub(1)
        .map(|i| &non_list[i])
        .filter(|&r| is_break(r))
        .or_else(|| list.iter().find(|&r| is_break(r)))
}

/// The first skippable range that genuinely encloses `boundary`: it contains the
/// offset and is not a symmetric-quote mispairing (which only looks like containment).
/// `Some` means the terminator at `boundary` should be suppressed rather than split on.
fn containing_range<'r, L: Language + ?Sized>(
    lang: &L,
    paragraph: &str,
    boundary: usize,
    match_start: usize,
    match_end: usize,
    ranges: &'r [SkippableRange],
    region: NonListRegion,
) -> Option<&'r SkippableRange> {
    let is_break = |range: &SkippableRange| {
        range.contains(boundary)
            && !lang.is_symmetric_quote_mispairing(paragraph, range, match_start, match_end)
    };

    if region.binary_search {
        select_containing_binary(ranges, region.len, boundary, is_break)
    } else {
        ranges.iter().find(|&r| is_break(r))
    }
}

/// The offset just past `range`'s closer where the sentence resumes, when `boundary`
/// is a terminator at the range's inner edge (e.g. the . in "... end."), otherwise return `None`.
fn inner_terminator_boundary<L: Language + ?Sized>(
    lang: &L,
    paragraph: &str,
    range: &SkippableRange,
    boundary: usize,
) -> Option<usize> {
    if !range.is_inner_terminator(paragraph, boundary) {
        return None;
    }

    let next_word = lang.get_next_word_approx(paragraph, range.end);
    let extend = lang.get_boundary_extend(next_word);

    (extend >= 0).then(|| range.end + extend as usize)
}

/// Push a skippable range for each list-item line span (the last to `paragraph_len`)
/// so a terminator inside an item does not split it.
fn add_list_item_ranges(
    skippable_ranges: &mut Vec<SkippableRange>,
    list_starts: &[usize],
    paragraph_len: usize,
) {
    for pair in list_starts.windows(2) {
        skippable_ranges.push(SkippableRange::new(
            pair[0],
            pair[1],
            SkippableRangeType::ListItem,
        ));
    }

    if let Some(&last) = list_starts.last() {
        skippable_ranges.push(SkippableRange::new(
            last,
            paragraph_len,
            SkippableRangeType::ListItem,
        ));
    }
}

/// Add each list item line start as a sentence boundary, then sort and dedup.
fn merge_list_item_boundaries(sentence_boundaries: &mut Vec<usize>, list_starts: &[usize]) {
    if list_starts.is_empty() {
        return;
    }

    for &start in list_starts {
        if start > 0 {
            sentence_boundaries.push(start);
        }
    }

    sentence_boundaries.sort_unstable();
    sentence_boundaries.dedup();
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
        let text_len = text.len();
        let capacity = (text_len / 50).max(1);
        let mut boundaries = Vec::with_capacity(capacity);
        let mut scratch = ParagraphScratch::with_capacity(capacity);
        let regex = self.get_sentence_break_regex();

        // Walk each paragraph paired with its trailing separator (`None` after the last
        // paragraph). `para_start` / `char_offset` are the running byte / character
        // cursors, tracked separately for correct multi-byte UTF-8 handling ("日本語"
        // is 3 characters but 9 bytes).
        let (mut para_start, mut char_offset) = (0usize, 0usize);
        let trailing_separators = paragraph_breaks(text)
            .map(Some)
            .chain(std::iter::once(None));

        for trailing_separator in trailing_separators {
            let para_end = trailing_separator.map_or(text_len, |(sep_start, _)| sep_start);
            let paragraph = &text[para_start..para_end];

            collect_sentence_breaks(self, paragraph, regex, &mut scratch);
            push_paragraph_sentences(
                paragraph,
                para_start,
                &scratch.sentence_boundaries,
                &mut char_offset,
                &mut boundaries,
            );

            // Emit the separator that follows this paragraph (none after the last).
            if let Some((sep_start, sep_end)) = trailing_separator {
                push_separator_boundary(
                    &mut boundaries,
                    &text[sep_start..sep_end],
                    sep_start,
                    sep_end,
                    &mut char_offset,
                );

                para_start = sep_end;
            }
        }

        boundaries
    }

    /// Segments `text` into sentence and paragraph separator slices.
    /// Emits slices directly instead of building per-sentence index/symbol metadata.
    fn segment<'a>(&self, text: &'a str) -> Vec<&'a str> {
        let text_len = text.len();
        let capacity = (text_len / 50).max(1);
        let mut sentences = Vec::with_capacity(capacity);
        let mut scratch = ParagraphScratch::with_capacity(capacity);
        let regex = self.get_sentence_break_regex();

        let mut para_start = 0usize;
        let trailing_separators = paragraph_breaks(text)
            .map(Some)
            .chain(std::iter::once(None));

        for trailing_separator in trailing_separators {
            let para_end = trailing_separator.map_or(text_len, |(sep_start, _)| sep_start);
            let paragraph = &text[para_start..para_end];

            collect_sentence_breaks(self, paragraph, regex, &mut scratch);

            for window in scratch.sentence_boundaries.windows(2) {
                let sentence = &paragraph[window[0]..window[1]];
                if !sentence.is_empty() {
                    sentences.push(sentence);
                }
            }

            if let Some((sep_start, sep_end)) = trailing_separator {
                let separator = &text[sep_start..sep_end];
                if !separator.is_empty() {
                    sentences.push(separator);
                }

                para_start = sep_end;
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

    /// When a lowercase/digit (or comma) follower, an ellipsis continuation, or a spaced `!`/`?`
    /// before a lowercase word occurs after a terminator, suppress the sentence break.
    fn terminator_continues(&self, matched: &str, head: &str, next_word_approx: &str) -> bool {
        if matched.chars().nth(1).is_some() {
            return self.is_ellipsis_continuation(next_word_approx)
                || (head.chars().next_back().is_some_and(|c| !c.is_whitespace())
                    && starts_with_ascii_lowercase_or_digit(next_word_approx));
        }

        self.continue_in_next_word(next_word_approx)
            // e.g., "Father Came Too ! is a British comedy film".
            || (matches!(matched, "!" | "?")
                && matches!(head.as_bytes().last(), Some(b' ' | b'\t'))
                && CONTINUE_AFTER_NONWORD_REGEX.is_match(next_word_approx))
    }

    /// Whether a `.` terminator should be suppressed.
    fn period_suppresses_boundary(
        &self,
        head: &str,
        last_word: &str,
        next_word_approx: &str,
    ) -> bool {
        let suppress = self.is_name_initial_for(head, last_word, next_word_approx)
            || self.is_abbreviation_for(last_word, ".");

        let marker = classify_trailing_marker(head, self.get_trailing_markers());
        if !suppress && marker.is_none() {
            return false;
        }

        let next_is_starter = self.next_word_is_sentence_starter(next_word_approx);
        let marker_bypass = marker.as_ref().is_some_and(|m| {
            marker_bypasses_suppression(m, next_word_approx, next_is_starter, self)
        });

        !marker_bypass
            && !self.should_override_abbrev_suppression_for(head, last_word, next_is_starter)
    }

    /// Analyzes a potential sentence boundary and determines the exact position where
    /// the sentence should end, or returns None if this shouldn't be a boundary.
    /// Considers abbreviations, exclamations, numbered references, and continuation patterns.
    /// This is the core logic that distinguishes true sentence boundaries from false positives
    /// like abbreviations or mid-sentence punctuation.
    fn find_boundary(&self, text: &str, start: usize, end: usize) -> Option<usize> {
        let head = &text[..start];
        let matched = &text[start..end];
        let next_word_approx = self.get_next_word_approx(text, end);

        // Gate the regex to only run if `[` is found
        if memchr::memchr(b'[', next_word_approx.as_bytes()).is_some()
            && let Some(m) = crate::constants::NUMBERED_REFERENCE_REGEX.find(next_word_approx)
        {
            return Some(end + m.end());
        }

        if self.terminator_continues(matched, head, next_word_approx) {
            return None;
        }

        let last_word = self.get_last_word(head);

        if matched == "." {
            if is_code_like_numbered_token(head, next_word_approx) {
                return None;
            }

            if self.period_suppresses_boundary(head, last_word, next_word_approx) {
                return None;
            }
        }

        if self.is_exclamation_for(last_word) {
            return None;
        }

        // Swallow any whitespace after the terminator into the boundary.
        // Replaces the `^\s+` regex.
        let trailing_ws = next_word_approx.len() - next_word_approx.trim_start().len();
        Some(end + trailing_ws)
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
    fn get_skippable_ranges(&self, text: &str, out: &mut Vec<SkippableRange>) {
        out.clear();

        // Fast path: ASCII `'`, backtick, and the `0xE3` lead byte are the only
        // chars in the ambiguous and CJK (`《》「」`) quote pairs.
        // If those are not present, only `open(?s:.*?)close` pairs can match.
        if memchr::memchr3(b'\'', b'`', 0xE3, text.as_bytes()).is_none() {
            scan_unambiguous_quotes(text, out);
        } else {
            for mat in QUOTES_REGEX.find_iter(text) {
                let pair = identify_quote_pair(&text[mat.start()..])
                    .expect("QUOTES_REGEX match must start with a known pair opener");

                out.push(SkippableRange::new_quote(mat.start(), mat.end(), pair));
            }

            append_space_padded_quote_pairs(text, out);
        }

        for mat in PARENS_REGEX.find_iter(text) {
            out.push(SkippableRange::new(
                mat.start(),
                mat.end(),
                SkippableRangeType::Parentheses,
            ));
        }

        for mat in EMAIL_REGEX.find_iter(text) {
            out.push(SkippableRange::new(
                mat.start(),
                mat.end(),
                SkippableRangeType::Email,
            ));
        }

        // Sort ranges by start position for more efficient lookups
        out.sort_unstable_by_key(|r| r.start);

        // Cache mispairing on each quote range for re-use
        populate_quote_mispairing(text, out);
    }
}

#[cfg(test)]
mod tests {
    use super::Language;
    use crate::languages::Japanese;

    #[test]
    fn get_boundary_extend_sums_run_in_bytes() {
        let lang = Japanese {};

        assert_eq!(lang.get_boundary_extend(". X"), 2);
        assert_eq!(lang.get_boundary_extend("。次"), 3);
        assert_eq!(lang.get_boundary_extend("。。次"), 6);
        assert_eq!(lang.get_boundary_extend(""), 0);
        assert_eq!(lang.get_boundary_extend(" foo"), -1);
    }
}
