// Some abbreviations like `a.m.`, `B.C.`, `Ph.D.`, `5 ft.`, `NE.` are not universally
// suppressible. They are usually markers trailing a digit or a name, and breaking a sentence
// at them correctly needs some semantic context. These definitions flag marker words with
// different handling policies to allow targeted carve-outs.
// For example, suppose `p.m` has a policy that breaks before an uppercase follower.
// `In the evening at 7 p.m. Tom wakes up.`
//  - No split. `In the evening at` are all fronting (adverbial) words, so the marker reads as mid-phrase.
// `The sun sets at 7 p.m. Tom wakes up then.`
//  - Splits into [`The sun sets at 7 p.m. `, `Tom wakes up then.`].
//  - `sun` is lowercase and not in the fronting list, while `Tom` is an uppercase follower, so the break stands.
use std::sync::LazyLock;

use super::fronting::{prefix_is_purely_fronting, word_before_marker_is_capitalised};
use super::language::Language;

/// Describes the policy for handling trailing markers.
/// See `trailing_markers/en.txt` for examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkerPolicy {
    /// Whether a digit follower forces a sentence break.
    pub digit_breaks: bool,
    /// Whether an uppercase non-starter follower forces a sentence break.
    /// Not useful for English, but might be for other languages like German.
    pub uppercase_breaks: bool,
}

pub struct MarkerDef {
    pub matcher: SuffixMatcher,
    pub policy: MarkerPolicy,
}

/// How a marker's suffix is recognized in the tail.
pub struct SuffixMatcher {
    pub suffix: &'static str,
    pub ignore_case: bool,
    pub digit_only: bool,
}

impl SuffixMatcher {
    /// The text before the suffix when `head` ends with it (the reverse match), else `None`.
    /// Case-sensitive markers use `str::strip_suffix`. Case-insensitive ones compare the tail bytes.
    #[inline]
    fn strip<'h>(&self, head: &'h str) -> Option<&'h str> {
        if self.ignore_case {
            let idx = head.len().checked_sub(self.suffix.len())?;
            head.as_bytes()[idx..]
                .eq_ignore_ascii_case(self.suffix.as_bytes())
                .then(|| &head[..idx])
        } else {
            head.strip_suffix(self.suffix)
        }
    }
}

pub struct MarkerTable {
    markers: Box<[MarkerDef]>,
    /// Rejects tails whose final two bytes match no marker, so the linear `markers` scan runs only for real candidates.
    two_byte_filter: TwoByteFilter,
}

impl MarkerTable {
    pub fn empty() -> &'static Self {
        static EMPTY: LazyLock<MarkerTable> = LazyLock::new(|| MarkerTable {
            markers: Box::from([]),
            two_byte_filter: TwoByteFilter::EMPTY,
        });

        &EMPTY
    }

    pub fn build(markers: Vec<MarkerDef>) -> Self {
        Self {
            two_byte_filter: build_two_byte_filter(&markers),
            markers: markers.into_boxed_slice(),
        }
    }
}

/// Folded (lowercased) last-two-byte lookup key. Case folding makes the filter case-insensitive.
/// Case-sensitive markers are re-checked precisely in `strip`.
#[inline]
fn two_byte_key(second_last: u8, last: u8) -> usize {
    ((second_last.to_ascii_lowercase() as usize) << 8) | last.to_ascii_lowercase() as usize
}

/// A 65536-bit set of `two_byte_key`.
struct TwoByteFilter([u64; 1024]);

impl TwoByteFilter {
    const EMPTY: Self = Self([0; 1024]);

    #[inline]
    fn insert(&mut self, key: usize) {
        self.0[key >> 6] |= 1u64 << (key & 63);
    }

    #[inline]
    fn contains(&self, key: usize) -> bool {
        self.0[key >> 6] & (1u64 << (key & 63)) != 0
    }
}

/// Marks every folded last two byte of every marker.
fn build_two_byte_filter(markers: &[MarkerDef]) -> TwoByteFilter {
    let mut filter = TwoByteFilter::EMPTY;

    for marker in markers {
        let suffix = marker.matcher.suffix.as_bytes();
        let last = *suffix.last().expect("marker suffix must be non-empty");

        match suffix {
            [.., second_last, _] => filter.insert(two_byte_key(*second_last, last)),
            _ => (0u8..=255).for_each(|hi| filter.insert(two_byte_key(hi, last))),
        }
    }

    filter
}

pub(crate) struct MarkerMatch<'a> {
    pub(crate) prefix: &'a str,
    pub(crate) def: &'static MarkerDef,
}

/// The suffix must be preceded by whitespace, a digit, or start of text, so `clause 5.a.m` doesn't match `a.m`.
fn strip_marker_suffix<'h>(
    head_trimmed: &'h str,
    markers: &'static [MarkerDef],
) -> Option<MarkerMatch<'h>> {
    markers.iter().find_map(|marker| {
        let prefix = marker.matcher.strip(head_trimmed)?;

        let predecessor_ok = if marker.matcher.digit_only {
            prefix
                .trim_end()
                .chars()
                .next_back()
                .is_some_and(|c| c.is_ascii_digit())
        } else {
            prefix
                .chars()
                .next_back()
                .is_none_or(|c| c.is_whitespace() || c.is_ascii_digit())
        };

        predecessor_ok.then_some(MarkerMatch {
            prefix,
            def: marker,
        })
    })
}

#[inline]
pub(crate) fn classify_trailing_marker<'h>(
    head: &'h str,
    table: &'static MarkerTable,
) -> Option<MarkerMatch<'h>> {
    let trimmed = head.trim_end();

    // A marker is at least two bytes, so a shorter head can't match one.
    // Reject on the folded last two bytes. Words whose final pair matches no marker never reach the scan.
    let [.., second_last, last] = trimmed.as_bytes() else {
        return None;
    };

    let key = two_byte_key(*second_last, *last);
    if !table.two_byte_filter.contains(key) {
        return None;
    }

    strip_marker_suffix(trimmed, &table.markers)
}

/// Decides whether a matched marker breaks the sentence (`true`) or keeps
/// suppressing the break (`false`), based on the next word's first char.
/// * Single-letter initials (`P.`) and lowercase/punctuation never break.
/// * A digit defers to `digit_breaks`.
/// * An uppercase follower breaks if it's a sentence starter, or if `uppercase_breaks` holds and fronting doesn't veto it.
pub(crate) fn marker_bypasses_suppression<L: Language + ?Sized>(
    marker: &MarkerMatch<'_>,
    next_word: &str,
    next_is_starter: bool,
    lang: &L,
) -> bool {
    let policy = marker.def.policy;
    let mut chars = next_word.trim_start().chars();
    let first = chars.next();

    // Single-letter capital initial (`P.D.T.`, `J. R. R.`) continues.
    if first.is_some_and(|c| c.is_ascii_uppercase()) && chars.next() == Some('.') {
        return false;
    }

    match first {
        Some(c) if c.is_ascii_digit() => policy.digit_breaks,
        Some(c) if c.is_uppercase() => {
            next_is_starter
                || (policy.uppercase_breaks
                    && !word_before_marker_is_capitalised(marker.prefix)
                    && !prefix_is_purely_fronting(marker.prefix, lang))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::super::language::Language;
    use super::{MarkerDef, MarkerMatch, MarkerPolicy, SuffixMatcher, marker_bypasses_suppression};

    struct StubLang;
    impl Language for StubLang {}

    const fn marker(uppercase_breaks: bool) -> MarkerDef {
        MarkerDef {
            matcher: SuffixMatcher {
                suffix: "p.m",
                ignore_case: true,
                digit_only: false,
            },
            policy: MarkerPolicy {
                digit_breaks: true,
                uppercase_breaks,
            },
        }
    }

    const PREFIX: &str = "the sun sets at 7 ";

    static CONT_MARKER: MarkerDef = marker(false);
    static BREAK_MARKER: MarkerDef = marker(true);

    #[test]
    fn uppercase_breaks_false_keeps_suppression() {
        let m = MarkerMatch {
            prefix: PREFIX,
            def: &CONT_MARKER,
        };
        assert!(!marker_bypasses_suppression(
            &m,
            "Tom wakes up",
            false,
            &StubLang
        ));
    }

    #[test]
    fn starter_override_wins_over_uppercase_breaks_false() {
        let m = MarkerMatch {
            prefix: PREFIX,
            def: &CONT_MARKER,
        };
        assert!(marker_bypasses_suppression(
            &m,
            "Tom wakes up",
            true,
            &StubLang
        ));
    }

    #[test]
    fn uppercase_breaks_true_breaks_on_uppercase_follower() {
        let m = MarkerMatch {
            prefix: PREFIX,
            def: &BREAK_MARKER,
        };
        assert!(marker_bypasses_suppression(
            &m,
            "Tom wakes up",
            false,
            &StubLang
        ));
    }
}
