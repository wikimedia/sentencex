// Quote handling
// Detecting quote ranges, classifying quote candidates, pairing space padded quotes the regex misses,
// tagging symmetric pair mispairings, and extending sentence boundaries past closers (both terminators
// sitting just inside a quote and orphan trailing closers).

use std::sync::LazyLock;

use crate::constants::QUOTE_CLOSERS_BY_LEN;
use crate::constants::QUOTE_PAIRS;
use crate::constants::QUOTES_REGEX;
use crate::constants::QuotePair;
use crate::constants::SPACE_AFTER_SEPARATOR;

use super::language::{Language, NonListRegion, SkippableRange, SkippableRangeType};

/// True iff it finds a `.[ \t]+[A-Z]` shape. Using memchr here instead of regex for speed.
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

/// True when `closer` is the closing token of a symmetric quote pair, e.g., `'`, where opener and closer symbols are the same.
/// Catch space padded cases when `QUOTES_REGEX` fails to pair them.
pub(crate) fn is_symmetric_quote_closer(closer: &str) -> bool {
    QUOTE_PAIRS
        .iter()
        .any(|p| p.open == p.close && p.close == closer)
}

/// True when the `closer` token at `boundary` in `paragraph` is an orphan. Either it has no matching opener earlier in the
/// paragraph, or it is a lone stray.
/// Asymmetric closers, e.g., `»`, `”`, are unambiguous and always orphan once `QUOTES_REGEX` has had a chance to pair.
/// Symmetric closers inspect the closer's distribution across the paragraph, ignoring ones consumed by a paired quote range
/// and contractions like `wasn't`.
fn is_orphan_closer(
    paragraph: &str,
    boundary: usize,
    closer: &'static str,
    skippable_ranges: &[SkippableRange],
    cache: &mut OrphanCloserPositions,
) -> bool {
    if !is_symmetric_quote_closer(closer) {
        return true;
    }

    let (before, total) = cache.before_and_total(paragraph, closer, boundary, skippable_ranges);
    let at_or_after = total - before;
    let unmatched_opener_before = before % 2 == 1;
    let lone_stray_at_boundary = before == 0 && at_or_after == 1;

    unmatched_opener_before || lone_stray_at_boundary
}

fn identify_quote_pair(span: &str) -> Option<&'static QuotePair> {
    QUOTE_PAIRS.iter().find(|p| span.starts_with(p.open))
}

/// Collect every quote `SkippableRange` in `text` into `out`. The fast path scans for non `'`/backtick/CJK pairs.
/// Full scan is via the regex. Additionally catches space padded `'…'`, `` `…` `` pairs.
pub(crate) fn collect_quote_ranges(text: &str, out: &mut Vec<SkippableRange>) {
    // Fast path: ASCII `'`, backtick, and the `0xE3` lead byte (for CJK `《》`,`「」`)
    if memchr::memchr3(b'\'', b'`', 0xE3, text.as_bytes()).is_none() {
        scan_unambiguous_quotes(text, out);
        return;
    }

    append_regex_quote_pairs(text, out);
    append_space_padded_quote_pairs(text, out);
}

/// Append every `QUOTES_REGEX` match in `text` as a quote range.
fn append_regex_quote_pairs(text: &str, out: &mut Vec<SkippableRange>) {
    for mat in QUOTES_REGEX.find_iter(text) {
        let pair = identify_quote_pair(&text[mat.start()..])
            .expect("QUOTES_REGEX match must start with a known pair opener");

        out.push(SkippableRange::new_quote(mat.start(), mat.end(), pair));
    }
}

/// Single char quote openers whose `open`/`close` contain no ASCII `'` or backtick. Map to `QuotePair`.
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

/// Fast path when the text has no `'`, backtick, or `0xE3` lead byte (CJK variants).
fn scan_unambiguous_quotes(text: &str, out: &mut Vec<SkippableRange>) {
    let bytes = text.as_bytes();
    let mut cursor = 0;

    // 0xC2 lead byte for guillemet chars like `«`. 0xE2 lead byte for curly / smart quotes.
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

pub(crate) fn is_symmetric_quote_range(range: &SkippableRange) -> bool {
    range.quote_pair.is_some_and(|p| p.open == p.close)
}

/// Last symmetric quote token's whole paragraph count parity.
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

/// True when the paragraph contains an odd number of the symmetric quote token that opens `range`.
/// An odd count has at least one orphan. When that orphan sits earlier than a real downstream opener,
/// `QUOTES_REGEX` mispairs this across a real sentence break.
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

/// The `parens` are sorted and disjoint. Find the one with byte offset `x`, `p.start < x < p.end`.
fn paren_containing(parens: &[SkippableRange], x: usize) -> Option<&SkippableRange> {
    parens
        .partition_point(|p| p.start < x)
        .checked_sub(1)
        .map(|i| &parens[i])
        .filter(|p| x < p.end)
}

/// True when `quote` partially overlaps any paren in `parens`, i.e., one part in, one part out.
/// Full containment returns false. `parens` must be sorted by start and disjoint.
fn quote_partially_overlaps_parens(quote: &SkippableRange, parens: &[SkippableRange]) -> bool {
    debug_assert!(
        parens.windows(2).all(|w| w[0].end <= w[1].start),
        "parens must be sorted and disjoint"
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

/// Trim leading whitespace from `s`. If a symmetric quote closer follows, peel one closer plus its trailing whitespace.
/// Used to look through a stray closing apostrophe when checking for a comma continuation, e.g., `. ' , Tim ...`.
pub(crate) fn peel_leading_symmetric_quote(s: &str) -> &str {
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
fn is_in_quote_range(ranges: &[SkippableRange], idx: usize) -> bool {
    let pos = ranges.partition_point(|r| r.start <= idx);
    ranges[..pos].iter().any(|r| r.is_quote() && idx < r.end)
}

/// Chars immediately before `idx` and immediately after `idx + token.len()`.
/// Return `(prev, next)`. Either side is `None` at the text boundary.
fn neighbors(text: &str, idx: usize, token: &str) -> (Option<char>, Option<char>) {
    (
        text[..idx].chars().next_back(),
        text[idx + token.len()..].chars().next(),
    )
}

/// True when the `token` at `idx` is a contraction, e.g., `wasn't`.
/// If sandwiched between two alphanumerics, they are not quotes.
fn is_contraction_quote(text: &str, idx: usize, token: &str) -> bool {
    let (prev, next) = neighbors(text, idx, token);
    prev.is_some_and(|c| c.is_alphanumeric()) && next.is_some_and(|c| c.is_alphanumeric())
}

fn is_quote_candidate(text: &str, idx: usize, token: &str, ranges: &[SkippableRange]) -> bool {
    !is_in_quote_range(ranges, idx) && !is_contraction_quote(text, idx, token)
}

/// True when `'` or `` ` `` at `idx` is preceded by start of text or whitespace and followed by whitespace.
fn is_opener_shape(text: &str, idx: usize, token: &str) -> bool {
    let (prev, next) = neighbors(text, idx, token);
    prev.is_none_or(char::is_whitespace) && next.is_some_and(char::is_whitespace)
}

/// True when `text[from..]` starts with one or more ASCII blanks followed by an ASCII uppercase letter.
fn starts_new_utterance(text: &str, from: usize) -> bool {
    let tail = &text[from..];
    let trimmed = tail.trim_start_matches([' ', '\t']);

    trimmed.len() < tail.len()
        && trimmed
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_uppercase)
}

/// True if there is a quote ahead with no inline sentence break inbetween.
fn candidate_opens_clean_span(
    text: &str,
    idx: usize,
    token: &str,
    ranges: &[SkippableRange],
) -> bool {
    if !is_opener_shape(text, idx, token) {
        return false;
    }

    let content_start = idx + token.len();
    text[content_start..]
        .match_indices(token)
        .map(|(rel, _)| content_start + rel)
        .find(|&y| is_quote_candidate(text, y, token, ranges))
        .is_some_and(|y| !has_possible_inline_sentence_break(&text[content_start..y]))
}

/// True when the `'` or `` ` `` candidates at `opener` and `closer` should pair into a quote range. `opener` must be opener shaped.
/// The pair is rejected only when the span looks like two back to back utterances, and the closer itself cleanly opens a further quote ahead.
fn quote_candidates_should_pair(
    text: &str,
    opener: usize,
    closer: usize,
    token: &str,
    ranges: &[SkippableRange],
) -> bool {
    if !is_opener_shape(text, opener, token) {
        return false;
    }

    let span = &text[opener + token.len()..closer];
    let back_to_back = starts_new_utterance(text, closer + token.len())
        && has_possible_inline_sentence_break(span);

    !back_to_back || !candidate_opens_clean_span(text, closer, token, ranges)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QuoteMispairing {
    #[default]
    None,
    Certain,
    Possible,
}

/// Tag each symmetric pair quote range with a mispairing label.
pub(crate) fn tag_quote_mispairing(paragraph: &str, ranges: &mut [SkippableRange]) {
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

/// True when this symmetric-pair quote range (`''…''`, `'…'`, `"…"`) is
/// a likely `QUOTES_REGEX` mispairing across a sentence break.
/// - Certain: range straddles a parens boundary. Override always
///   fires. A quote pair shouldn't cross a parens edge.
/// - Possible: paragraph has an odd token count (one or more
///   orphans). Override fires only if `[start, end)` also looks like
///   a strong sentence break.
/// - None: not a symmetric pair or evenly balanced. No override.
pub(crate) fn is_symmetric_quote_mispairing<L: Language + ?Sized>(
    lang: &L,
    paragraph: &str,
    range: &SkippableRange,
    start: usize,
    end: usize,
) -> bool {
    match range.quote_mispairing {
        QuoteMispairing::None => false,
        QuoteMispairing::Certain => true,
        QuoteMispairing::Possible => lang.has_strong_sentence_break(paragraph, start, end),
    }
}

/// Append `'…'`, `` `…` `` ranges that `QUOTES_REGEX` couldn't pair.
/// Guarded patterns require `\b` immediately after the opener, so space padded openers like
// `' word ` go unpaired even when they form a real `' … '` pair.
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
                && quote_candidates_should_pair(text, opener, idx, token, ranges)
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

/// True when the symmetric `closer` at `boundary` opens the next sentence rather than trailing the current one,
/// so the caller keeps the boundary and lets the closer join what follows.
fn closer_opens_next_sentence(
    paragraph: &str,
    boundary: usize,
    closer: &str,
    skippable_ranges: &[SkippableRange],
) -> bool {
    // A symmetric quote with whitespace or start of text on its left, leading into a capitalized word.
    // `Coast.'` would fail as the `.` isn't clean.
    let opener_shaped = is_symmetric_quote_closer(closer)
        && paragraph[..boundary]
            .chars()
            .next_back()
            .is_none_or(char::is_whitespace)
        && starts_new_utterance(paragraph, boundary + closer.len());

    // The same token also needs to have already formed a real pair earlier. It's a signal it's an opener,
    // not a stray closer, e.g.,  `We do ? ''` has no earlier opener.
    opener_shaped
        && skippable_ranges.iter().any(|r| {
            r.end <= boundary
                && is_symmetric_quote_range(r)
                && paragraph[r.start..].starts_with(closer)
        })
}

/// Records in ascending order where symmetric closing quote tokens appear in a paragraph, e.g., every `'`.
/// Only real quote candidates are kept. Contraction apostrophes and marks already inside a paired quote are skipped.
/// Allows `is_orphan_closer` to count how many fall before/after a boundary with binary search, instead of rescanning
/// the paragraph every call.
#[derive(Default)]
pub(crate) struct OrphanCloserPositions {
    token: Option<&'static str>,
    positions: Vec<usize>,
}

impl OrphanCloserPositions {
    pub(crate) fn reset(&mut self) {
        self.token = None;
    }

    /// Returns `(before, total)`, where
    /// `before`: how many candidate occurrences of token sit strictly before boundary (byte offset).
    /// `total`: how many candidate occurrences there are in the whole paragraph.
    fn before_and_total(
        &mut self,
        paragraph: &str,
        token: &'static str,
        boundary: usize,
        ranges: &[SkippableRange],
    ) -> (usize, usize) {
        if self.token != Some(token) {
            self.positions.clear();

            self.positions.extend(
                paragraph
                    .match_indices(token)
                    .map(|(idx, _)| idx)
                    .filter(|&idx| is_quote_candidate(paragraph, idx, token, ranges)),
            );

            self.token = Some(token);
        }

        let before = self.positions.partition_point(|&p| p < boundary);
        (before, self.positions.len())
    }
}

/// True when some skippable range starts exactly at `boundary`.
fn range_starts_at(ranges: &[SkippableRange], boundary: usize, region: NonListRegion) -> bool {
    // Binary search when the list is large
    if !region.binary_search {
        return ranges.iter().any(|r| r.start == boundary);
    }

    // Linear scan when small
    let non_list = &ranges[..region.len];
    let idx = non_list.partition_point(|r| r.start < boundary);
    non_list.get(idx).is_some_and(|r| r.start == boundary)
        || ranges[region.len..].iter().any(|r| r.start == boundary)
}

/// True if `boundary` lies just before this quote's closing mark.
/// Opinionated behaviour, but it can help to resolve some real world cases with erroneous
/// or ambiguous punctuation placement.
/// Example: `He said "Hello."`, the `.` is immediately followed by `"`.
/// The sentence should extend past the quote rather than break.
fn is_inner_terminator(range: &SkippableRange, text: &str, boundary: usize) -> bool {
    if !range.is_quote() || boundary >= range.end {
        return false;
    }

    let head = &text[..range.end];
    QUOTE_CLOSERS_BY_LEN
        .iter()
        .any(|c| head.ends_with(*c) && boundary + c.len() == range.end)
}

/// The offset just past `range`'s closer where the sentence resumes, when `boundary`
/// is a terminator at the range's inner edge (e.g. the . in "... end."), otherwise return `None`.
pub(crate) fn inner_terminator_boundary<L: Language + ?Sized>(
    lang: &L,
    paragraph: &str,
    range: &SkippableRange,
    boundary: usize,
) -> Option<usize> {
    if !is_inner_terminator(range, paragraph, boundary) {
        return None;
    }

    let next_word = lang.get_next_word_approx(paragraph, range.end);
    lang.get_boundary_extend(next_word)
        .map(|extend| range.end + extend)
}

/// If `boundary` sits at an orphan trailing quote closer, e.g. `.'` with no matching opener captured by `QUOTES_REGEX`,
/// advance past the closer, trailing whitespace, stranded terminators that otherwise form a sentence.
/// Otherwise return `boundary` unchanged.
pub(crate) fn extend_past_orphan_closer<L: Language + ?Sized>(
    lang: &L,
    paragraph: &str,
    boundary: usize,
    skippable_ranges: &[SkippableRange],
    non_list_region: NonListRegion,
    orphan_closers: &mut OrphanCloserPositions,
) -> usize {
    // If the next char opens a known quoted range, that quote belongs to the upcoming sentence, do nothing.
    if range_starts_at(skippable_ranges, boundary, non_list_region) {
        return boundary;
    }

    // Try to find an orphan closer starting at `boundary`. Longest first so `''` wins over `'`.
    let mut found = None;
    for c in QUOTE_CLOSERS_BY_LEN.iter() {
        if paragraph[boundary..].starts_with(c)
            && is_orphan_closer(paragraph, boundary, c, skippable_ranges, orphan_closers)
        {
            found = Some(c);
            break;
        }
    }

    let Some(closer) = found else {
        return boundary;
    };

    // If this orphan closer is really the opener of the next sentence, do nothing.
    if closer_opens_next_sentence(paragraph, boundary, closer, skippable_ranges) {
        return boundary;
    }

    let advance_past_space = |pos: usize| {
        SPACE_AFTER_SEPARATOR
            .find(&paragraph[pos..])
            .map_or(pos, |m| pos + m.end())
    };

    let mut boundary = advance_past_space(boundary + closer.len());
    let sentence_break_regex = lang.get_sentence_break_regex();

    // Absorb any stranded terminators
    while let Some(m) = sentence_break_regex
        .find(&paragraph[boundary..])
        .filter(|m| m.start() == 0)
    {
        boundary = advance_past_space(boundary + m.end());
    }

    boundary
}
