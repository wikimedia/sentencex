mod am;
mod ar;
mod bg;
mod bn;
mod ca;
mod da;
mod de;
mod el;
mod en;
mod es;
mod fallbacks;
mod fi;
mod fr;
mod gu;
mod hi;
mod hy;
mod it;
mod ja;
mod kk;
mod kn;
mod language;
mod list_markers;
mod ml;
mod mr;
mod my;
mod nl;
mod pa;
mod pl;
mod pt;
mod ru;
mod sk;
mod ta;
mod te;
mod uk;

pub use am::Amharic;
pub use ar::Arabic;
pub use bg::Bulgarian;
pub use bn::Bengali;
pub use ca::Catalan;
pub use da::Danish;
pub use de::German;
pub use el::Greek;
pub use en::English;
pub use es::Spanish;
pub use fallbacks::get_fallbacks;
pub use fi::Finnish;
pub use fr::French;
pub use gu::Gujarati;
pub use hi::Hindi;
pub use hy::Armenian;
pub use it::Italian;
pub use ja::Japanese;
pub use kk::Kazakh;
pub use kn::Kannada;
pub use language::Language;
pub use ml::Malayalam;
pub use mr::Marathi;
pub use my::Burmese;
pub use nl::Dutch;
pub use pa::Punjabi;
pub use pl::Polish;
pub use pt::Portuguese;
pub use ru::Russian;
pub use sk::Slovak;
pub use ta::Tamil;
pub use te::Telugu;
pub use uk::Ukrainian;

use rustc_hash::FxHashSet;

/// Parse one or more bundled word-list files into a deduplicated set.
/// Lines are trimmed; blank lines and `//` line comments are dropped.
pub(crate) fn parse_word_list<'a>(sources: impl IntoIterator<Item = &'a str>) -> FxHashSet<String> {
    sources
        .into_iter()
        .flat_map(str::lines)
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(String::from)
        .collect()
}

pub(crate) fn parse_abbreviation_list<'a>(
    sources: impl IntoIterator<Item = &'a str>,
) -> FxHashSet<String> {
    parse_word_list(sources)
        .iter()
        .map(|s| s.to_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    pub fn run_language_tests<T: Language>(language: T, test_file: &str) {
        let raw = fs::read_to_string(test_file).expect("Failed to read test file");
        let content = raw.replace("\r\n", "\n");
        let test_cases: Vec<&str> = content.split("===").collect();

        for case in test_cases {
            // Explicit '#' comment handling. This lets fixture files include
            // section headers and structural comments anywhere — not just at
            // the very top of the file — without silently skipping cases that
            // happen to share a chunk with leading comments.
            let cleaned: String = case
                .lines()
                .filter(|line| !line.trim_start().starts_with('#'))
                .collect::<Vec<_>>()
                .join("\n");

            let cleaned = cleaned.trim();
            if cleaned.is_empty() {
                continue;
            }

            let parts: Vec<&str> = cleaned
                .split("---")
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .collect();

            if parts.len() != 2 {
                continue; // Skip malformed test cases
            }

            // Comparison normalises whitespace on both sides — any run of
            // whitespace (spaces, tabs, newlines) collapses to a single
            // space.
            let normalise = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
            let input = parts[0];

            let expected: Vec<String> = parts[1]
                .lines()
                .map(|s| normalise(&s))
                .filter(|s| !s.is_empty())
                .collect();

            let result = language.segment(&input);

            let actual: Vec<String> = result
                .iter()
                .map(|item| normalise(item))
                .filter(|s| !s.is_empty())
                .collect();

            assert_eq!(actual, expected, "Failed for input: \n{}", input);
        }
    }
}
