// Fronting-phrase detection: "does the text before this marker look like an adverbial lead?",

use crate::constants::{QUOTE_PAIRS, QUOTES_REGEX};

use super::language::{Language, abbreviation_set_contains};

/// True iff trailing word of `prefix` after stripping starts uppercase.
pub(crate) fn word_before_marker_is_capitalised(prefix: &str) -> bool {
    prefix
        .trim_end_matches(|c: char| c.is_ascii_digit() || c == ':' || c.is_whitespace())
        .rsplit(char::is_whitespace)
        .next()
        .and_then(|w| w.chars().next())
        .is_some_and(|c| c.is_ascii_uppercase())
}

/// Any quote-pair opener begins with this byte. Used by `prefix_is_purely_fronting` as a fast check.
const QUOTE_OPENER_FIRST_BYTES: [bool; 256] = {
    let mut t = [false; 256];
    let mut i = 0;

    while i < QUOTE_PAIRS.len() {
        let open_bytes = QUOTE_PAIRS[i].open.as_bytes();
        t[open_bytes[0] as usize] = true;
        i += 1;
    }

    t
};

/// True iff `prefix` is purely a fronting (adverbial) lead-in, so the marker sits
/// mid-phrase rather than ending a sentence, e.g. `On Jan. 5, at 6 a.m.`. Splitting
/// on whitespace, commas, semicolons, and dashes, every token must be a fronting-list
/// word, an abbreviation, a number/time, digits, `:`, or capitalised. Spans inside
/// paired quotes are skipped.
/// Returns `false` when the language has no fronting list.
pub(crate) fn prefix_is_purely_fronting<L: Language + ?Sized>(prefix: &str, lang: &L) -> bool {
    const EM_DASH: char = '\u{2014}';
    const EN_DASH: char = '\u{2013}';

    let fronting = lang.get_fronting_words();
    if fronting.is_empty() {
        return false;
    }

    let abbrevs = lang.get_abbreviations();

    let segment_is_fronting = |seg: &str| {
        seg.split(|c: char| {
            c.is_whitespace() || c == ',' || c == ';' || c == EM_DASH || c == EN_DASH
        })
        .all(|tok| {
            let bare = tok.trim_end_matches('.');
            bare.is_empty()
                || bare.chars().all(|c| c.is_ascii_digit() || c == ':')
                || bare.chars().next().is_some_and(|c| c.is_uppercase())
                || abbreviation_set_contains(fronting, bare)
                || abbreviation_set_contains(abbrevs, bare)
        })
    };

    // Skip when no quote pair opener.
    if !prefix.bytes().any(|b| QUOTE_OPENER_FIRST_BYTES[b as usize]) {
        return segment_is_fronting(prefix);
    }

    let mut cursor = 0;
    for mat in QUOTES_REGEX.find_iter(prefix) {
        if !segment_is_fronting(&prefix[cursor..mat.start()]) {
            return false;
        }

        cursor = mat.end();
    }

    segment_is_fronting(&prefix[cursor..])
}
