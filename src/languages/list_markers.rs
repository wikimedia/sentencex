// List-item line/inline detector.
//
// Scans each paragraph once via memchr-driven line iteration, classifying
// markers at both line starts and inline positions (after a whitespace run).
// A "list = a sequence" sibling rule with a single winning family per paragraph
// keeps false positives down. The caller uses the returned offsets to emit
// sentence boundaries and to mark each item span as `SkippableRange::ListItem`.
//
// For performance reasons, char is used when unicode is unavoidable, and
// byte parsing everywhere else.

use strum::EnumCount;

const UNICODE_BULLETS: &[char] = &[
    '•', '◦', '▪', '▫', '■', '□', '●', '○', '⁃', '⁌', '⁍', '◆', '◇', '★', '☆', '➤', '➢', '➣', '▶',
    '▸', '►',
];

/// Minimum byte distance between two same-family candidates before the
/// second is kept. Real list items carry content between markers, so
/// genuine siblings sit far apart; tightly-packed repeats are prose
/// patterns (e.g. `e. e. cummings` - two `LetterDot` hits ~3 bytes apart).
/// 4 is the smallest value that drops those while still admitting short
/// real items like `1. x` (gap 4).
const MIN_GAP_BYTES: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumCount)]
enum MarkerFamily {
    Tier1,       // unicode bullets, parenthesised forms — fire on a single line-start match
    Bullet,      // * + - – —
    Numeric,     // 1. 1) 1.) 23.
    LetterParen, // a) A) a.) A.)
    LetterDot,   // a. b. (lowercase only)
    Roman,       // ii. iii) ii.) (≥2 chars, plus single-letter promoted via sibling rule)
}

impl MarkerFamily {
    fn idx(self) -> usize {
        self as usize
    }

    /// ASCII bullets (`*`, `+`, `-`) have no closing punctuation, so inline
    /// they are indistinguishable from parenthetical or hyphen uses
    /// ("Su - 24", "fast - track"). Two such uses would otherwise satisfy the
    /// sibling rule and emit false boundaries. Line-start bullets go through
    /// `classify_line` and are unaffected. Symmetric with the en/em-dash
    /// exclusion in `match_ascii_bullet`.
    fn allowed_inline(self) -> bool {
        !matches!(self, MarkerFamily::Bullet)
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
    pos: usize,
    family: MarkerFamily,
    first_byte: u8,
    line_start: bool,
}

impl Candidate {
    fn line_start(pos: usize, hit: MarkerHit) -> Self {
        Self {
            pos,
            family: hit.family,
            first_byte: hit.first_byte,
            line_start: true,
        }
    }

    fn inline(pos: usize, hit: MarkerHit) -> Self {
        Self {
            pos,
            family: hit.family,
            first_byte: hit.first_byte,
            line_start: false,
        }
    }

    /// A single-letter `a.`/`a)` candidate whose letter could read as Roman
    /// (i, v, x, l, c, d, m). Used by `promote_roman_letters`.
    fn is_roman_letter_shape(&self) -> bool {
        matches!(
            self.family,
            MarkerFamily::LetterDot | MarkerFamily::LetterParen
        ) && is_roman_byte(self.first_byte)
    }
}

#[derive(Debug, Clone, Copy)]
struct MarkerHit {
    family: MarkerFamily,
    first_byte: u8,
}

/// Returns byte offsets (within `paragraph`) of accepted list-item starts,
/// in source order with no duplicates.
pub(crate) fn detect_list_items(paragraph: &str) -> Vec<usize> {
    let bytes = paragraph.as_bytes();
    let mut candidates: Vec<Candidate> = Vec::with_capacity((paragraph.len() / 50).max(1));
    let mut line_start = 0usize;

    for nl_pos in memchr::memchr_iter(b'\n', bytes) {
        scan_line(paragraph, line_start, nl_pos + 1, &mut candidates);
        line_start = nl_pos + 1;
    }

    if line_start < bytes.len() {
        scan_line(paragraph, line_start, bytes.len(), &mut candidates);
    }

    finalise(candidates)
}

fn scan_line(text: &str, line_start: usize, line_end: usize, out: &mut Vec<Candidate>) {
    let bytes = text.as_bytes();
    let content_start = first_non_ws(bytes, line_start, line_end);

    if let Some(hit) = classify_line(&text[content_start..line_end]) {
        out.push(Candidate::line_start(line_start, hit));
    }

    // Each whitespace run is a candidate boundary; the byte after it may
    // begin an inline list marker. memchr2 jumps between runs.
    let mut cursor = content_start;
    while let Some(rel) = memchr::memchr2(b' ', b'\t', &bytes[cursor..line_end]) {
        let pos = first_non_ws(bytes, cursor + rel, line_end);
        if pos >= line_end || matches!(bytes[pos], b'\n' | b'\r') {
            break;
        }

        if let Some(hit) = classify_marker_at(&text[pos..line_end]) {
            out.push(Candidate::inline(pos, hit));
        }

        cursor = pos;
    }
}

/// Index of the first non-(space|tab) byte in `bytes[start..end]`, or `end`.
fn first_non_ws(bytes: &[u8], start: usize, end: usize) -> usize {
    let mut i = start;

    while i < end && is_horiz_ws(bytes[i]) {
        i += 1;
    }

    i
}

/// Classify the start of a line (with leading indent already stripped).
/// Requires a marker followed by at least one space and real content.
fn classify_line(line: &str) -> Option<MarkerHit> {
    let first_byte = *line.as_bytes().first()?;
    let (family, len) = consume_marker(line)?;
    next_content_char(&line[len..])?;
    Some(MarkerHit { family, first_byte })
}

/// Classify a marker found inline (after a whitespace run). Stricter than
/// `classify_line` because inline markers compete with prose punctuation.
fn classify_marker_at(s: &str) -> Option<MarkerHit> {
    let first_byte = *s.as_bytes().first()?;
    let (family, len) = consume_marker(s)?;

    if is_bare_dot_closer(s.as_bytes(), family, len) {
        return None;
    }

    let next = next_content_char(&s[len..])?;
    if next.is_lowercase() {
        return None;
    }

    if !family.allowed_inline() {
        return None;
    }

    Some(MarkerHit { family, first_byte })
}

/// Bare-dot closers (`1.`, `a.`, `ii.`) collide with sentence-ending periods
/// in prose. Inline markers must use `)` or `.)`. Line-start markers are
/// unaffected — the line break itself is the structural signal.
fn is_bare_dot_closer(bytes: &[u8], family: MarkerFamily, marker_len: usize) -> bool {
    bytes[marker_len - 1] == b'.'
        && matches!(
            family,
            MarkerFamily::Numeric | MarkerFamily::LetterDot | MarkerFamily::Roman
        )
}

/// Returns the first non-space, non-tab character after `after_marker`, only
/// if at least one space/tab is present and the resulting character is real
/// content (not a line terminator).
fn next_content_char(after_marker: &str) -> Option<char> {
    let bytes = after_marker.as_bytes();
    let n = first_non_ws(bytes, 0, bytes.len());
    if n == 0 {
        return None;
    }

    let c = after_marker[n..].chars().next()?;
    (c != '\n' && c != '\r').then_some(c)
}

fn consume_marker(s: &str) -> Option<(MarkerFamily, usize)> {
    // Order encodes priority. Tier 1 first; multi-char roman before single-letter
    // `[a-z]\.` so `ii.` isn't classified as `LetterDot`.
    None.or_else(|| match_unicode_bullet(s).map(|n| (MarkerFamily::Tier1, n)))
        .or_else(|| match_paren_form(s).map(|n| (MarkerFamily::Tier1, n)))
        .or_else(|| match_roman(s).map(|n| (MarkerFamily::Roman, n)))
        .or_else(|| match_numeric(s).map(|n| (MarkerFamily::Numeric, n)))
        .or_else(|| match_ascii_bullet(s).map(|n| (MarkerFamily::Bullet, n)))
        .or_else(|| match_letter_paren(s).map(|n| (MarkerFamily::LetterParen, n)))
        .or_else(|| match_letter_dot(s).map(|n| (MarkerFamily::LetterDot, n)))
}

fn match_unicode_bullet(s: &str) -> Option<usize> {
    let c = s.chars().next()?;
    UNICODE_BULLETS.contains(&c).then(|| c.len_utf8())
}

// En/em dashes (`–`/`—`) are intentionally NOT included: they collide with
// parenthetical dashes in prose (e.g. Kazakh "(1905 — 11)") where the
// sibling rule would falsely activate a list. Real `–`/`—` line-start
// bullets are rare; users wanting them can use `*` or `-` instead.
fn match_ascii_bullet(s: &str) -> Option<usize> {
    match s.as_bytes().first()? {
        b'*' | b'+' | b'-' => Some(1),
        _ => None,
    }
}

fn match_numeric(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    let n = b.iter().take_while(|c| c.is_ascii_digit()).count();
    if n == 0 {
        return None;
    }

    closer_len_at(b, n).map(|cl| n + cl)
}

fn match_roman(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    let n = b.iter().take_while(|&&c| is_roman_byte(c)).count();
    if n < 2 {
        return None;
    }

    closer_len_at(b, n).map(|cl| n + cl)
}

// Accepts `a)` and `a.)` but not `a.` alone (that's LetterDot's territory).
fn match_letter_paren(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    if b.len() < 2 || !b[0].is_ascii_alphabetic() {
        return None;
    }

    match b[1] {
        b')' => Some(2),
        b'.' if b.get(2) == Some(&b')') => Some(3),
        _ => None,
    }
}

fn match_letter_dot(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    (b.len() >= 2 && b[0].is_ascii_lowercase() && b[1] == b'.').then_some(2)
}

// (1), (12), (a), (A), (ii), (iv) — short, unpadded inners only.
// Padded inners (`( 1894 )`) and 3+ digit inners (`(1894)`) are prose
// year/date citations, not list markers.
fn match_paren_form(s: &str) -> Option<usize> {
    let inside = s.strip_prefix('(')?;
    let close = inside.find(')')?;

    // Safe since close lands on a valid utf8 boundary
    let inner = &inside.as_bytes()[..close];

    let short_numeric = (1..=2).contains(&inner.len()) && inner.iter().all(|b| b.is_ascii_digit());
    let single_letter = inner.len() == 1 && inner[0].is_ascii_alphabetic();
    let roman = !inner.is_empty() && inner.iter().all(|&b| is_roman_byte(b));

    (short_numeric || single_letter || roman).then_some(close + 2)
}

/// Length of marker closer at byte position `pos` — `.`, `)`, or `.)`.
fn closer_len_at(b: &[u8], pos: usize) -> Option<usize> {
    match b.get(pos)? {
        b'.' if b.get(pos + 1) == Some(&b')') => Some(2),
        b'.' | b')' => Some(1),
        _ => None,
    }
}

fn is_horiz_ws(b: u8) -> bool {
    b == b' ' || b == b'\t'
}

fn is_roman_byte(b: u8) -> bool {
    matches!(
        b.to_ascii_lowercase(),
        b'i' | b'v' | b'x' | b'l' | b'c' | b'd' | b'm'
    )
}

fn finalise(mut candidates: Vec<Candidate>) -> Vec<usize> {
    if candidates.is_empty() {
        return Vec::new();
    }

    sort_and_dedup(&mut candidates);
    promote_roman_letters(&mut candidates);
    gap_prune_per_family(&mut candidates);

    let Some(winner) = pick_winner(&candidates) else {
        return Vec::new();
    };

    candidates
        .into_iter()
        .filter(|c| c.family == winner)
        .map(|c| c.pos)
        .collect()
}

/// Sort by position; on ties (rare; degenerate inputs only) prefer line-start.
/// `!line_start` gives `false < true`, so `true` sorts first.
fn sort_and_dedup(candidates: &mut Vec<Candidate>) {
    candidates.sort_by_key(|c| (c.pos, !c.line_start));
    candidates.dedup_by_key(|c| c.pos);
}

/// If any multi-char Roman exists, reclassify single-letter roman-shaped
/// LetterDot/LetterParen candidates as Roman so they count as siblings
/// (handles `i.) ... ii.)`).
fn promote_roman_letters(candidates: &mut [Candidate]) {
    if !candidates.iter().any(|c| c.family == MarkerFamily::Roman) {
        return;
    }

    for c in candidates {
        if c.is_roman_letter_shape() {
            c.family = MarkerFamily::Roman;
        }
    }
}

/// Drop candidates too close to the previous match *of the same family*.
/// A real list item carries content; `e. e. cummings` (LetterDot gap 3) does
/// not. Run BEFORE family selection so sibling counts reflect the pruned set.
fn gap_prune_per_family(candidates: &mut Vec<Candidate>) {
    let mut family_last = [usize::MAX; MarkerFamily::COUNT];

    candidates.retain(|c| {
        let last = family_last[c.family.idx()];
        let keep = last == usize::MAX || c.pos - last >= MIN_GAP_BYTES;

        if keep {
            family_last[c.family.idx()] = c.pos;
        }

        keep
    });
}

/// Choose the winning family for the paragraph, or `None` for "no list".
///
/// Tier 1 (unicode bullets, parenthesised forms) wins on a single line-start
/// match or two matches anywhere. Otherwise the Tier 2 family with the most
/// matches wins, with the earliest first match breaking ties.
fn pick_winner(candidates: &[Candidate]) -> Option<MarkerFamily> {
    let mut counts = [0u32; MarkerFamily::COUNT];
    let mut first_pos = [usize::MAX; MarkerFamily::COUNT];
    let mut tier1_at_line_start = false;

    for c in candidates {
        let i = c.family.idx();
        counts[i] += 1;
        if first_pos[i] == usize::MAX {
            first_pos[i] = c.pos;
        }

        if c.family == MarkerFamily::Tier1 && c.line_start {
            tier1_at_line_start = true;
        }
    }

    if tier1_at_line_start || counts[MarkerFamily::Tier1.idx()] >= 2 {
        return Some(MarkerFamily::Tier1);
    }

    const TIER2: &[MarkerFamily] = &[
        MarkerFamily::Bullet,
        MarkerFamily::Numeric,
        MarkerFamily::LetterParen,
        MarkerFamily::LetterDot,
        MarkerFamily::Roman,
    ];

    TIER2
        .iter()
        .copied()
        .filter(|&f| counts[f.idx()] >= 2)
        .max_by_key(|&f| (counts[f.idx()], std::cmp::Reverse(first_pos[f.idx()])))
}

#[cfg(test)]
mod fixture {
    use crate::languages::English;
    use crate::languages::tests::run_language_tests;

    #[test]
    fn segments_lists() {
        run_language_tests(English {}, "tests/lists.txt");
    }
}

// Unit tests here cover detector-internal policy that the end-to-end fixture
// in `tests/lists.txt` cannot distinguish: cases where the detector must
// return an empty result (so the segmenter falls back to terminator-driven
// segmentation) and cases that test which family wins. Positive line-start /
// inline-paren shapes belong in `tests/lists.txt`.
#[cfg(test)]
mod tests {
    use super::*;

    fn detect(s: &str) -> Vec<usize> {
        detect_list_items(s)
    }

    // Detector-must-be-silent cases. Lists.txt sees the post-pipeline
    // segments, which can match terminator-driven output even if the detector
    // wrongly fires; these assert the detector itself returns nothing.

    #[test]
    fn inline_numeric_dot_only_not_handled() {
        // Bare-dot closer inline is ambiguous with a wrapped prose sentence
        // ending in `...Foo 1.`.
        let starts = detect("1. The first item. 2. The second item.");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn inline_letter_dot_not_handled() {
        // Bare-dot closer + lowercase letter collides with initials
        // (e. e. cummings) and date/abbrev shapes.
        let starts = detect("a. The first item b. The second item");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn inline_roman_dot_not_handled() {
        let starts = detect("ii. The first item iii. The second item");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn inline_year_parens_not_a_list() {
        let s = "Examples include 'Sonar Tari' ( 1894 ), 'Chitra' ( 1896 ), \
                 and 'Katha O Kahini' ( 1900 ).";
        assert!(detect(s).is_empty(), "got {:?}", detect(s));
        let s2 = "Works (1894), (1896), and (1900) are notable.";
        assert!(detect(s2).is_empty(), "got {:?}", detect(s2));
    }

    #[test]
    fn ee_cummings_no_segmentation() {
        // `e. e.` has zero non-marker content between markers → gap pruning
        // drops the second, leaving fewer than 2 siblings.
        let starts = detect("From\ne. e. cummings, with love.");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn uppercase_letter_dot_excluded() {
        let starts = detect("Reviewed by\nA. Smith and\nB. Jones.");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn lone_lowercase_letter_dot_no_siblings() {
        let starts = detect("The answer is a.\nFollow up later.");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn lone_numeric_after_wrap() {
        let starts = detect("The total was\n1. Two hundred dollars exactly.");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn eg_at_line_start() {
        let starts = detect("e.g. one\ne.g. two\n");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn marker_only_lines() {
        let starts = detect("*\n*\n*\n");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn no_markers_plain_prose() {
        let starts = detect("Hello world. This is a test. Three sentences here.");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    #[test]
    fn empty_paragraph() {
        let starts = detect("");
        assert!(starts.is_empty(), "got {starts:?}");
    }

    // Family-selection case: bullets must win over numeric decoration.
    #[test]
    fn inline_unicode_bullet_with_decoration() {
        let s = "• 9. The first item • 10. The second item";
        let starts = detect(s);
        assert_eq!(starts.len(), 2);
        assert_eq!(starts[0], 0);
        assert_eq!(&s[starts[1]..starts[1] + 3], "•");
    }
}
