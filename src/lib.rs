use languages::{
    Amharic, Arabic, Armenian, Bengali, Bulgarian, Burmese, Catalan, Danish, Dutch, English,
    Finnish, French, German, Greek, Gujarati, Hindi, Italian, Japanese, Kannada, Kazakh, Language,
    Malayalam, Marathi, Polish, Portuguese, Punjabi, Slovak, Spanish, Tamil,
};
use regex::Regex;
use std::sync::LazyLock;

mod constants;
pub mod languages;

use serde::Serialize;

static PARA_SPLIT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n[\r]*\n").unwrap());

pub fn fallback_language() -> Box<dyn Language> {
    Box::new(English {})
}

#[derive(Debug, Clone, Serialize)]
pub struct SentenceBoundary<'a> {
    pub start_index: usize,
    pub end_index: usize,
    pub start_byte: usize,
    pub end_byte: usize,
    pub text: &'a str,
    pub boundary_symbol: Option<String>,
    pub is_paragraph_break: bool,
}

pub fn language_factory(language_code: &str) -> Option<Box<dyn Language>> {
    let mut current_code = language_code;
    let mut visited = std::collections::HashSet::new();

    loop {
        if visited.contains(current_code) {
            return None; // cycle is detected
        } else {
            visited.insert(current_code);
        }

        match current_code {
            "am" => return Some(Box::new(Amharic {})),
            "ar" => return Some(Box::new(Arabic {})),
            "bg" => return Some(Box::new(Bulgarian {})),
            "bn" => return Some(Box::new(Bengali {})),
            "ca" => return Some(Box::new(Catalan {})),
            "da" => return Some(Box::new(Danish {})),
            "de" => return Some(Box::new(German {})),
            "en" => return Some(Box::new(English {})),
            "es" => return Some(Box::new(Spanish {})),
            "el" => return Some(Box::new(Greek {})),
            "gu" => return Some(Box::new(Gujarati {})),
            "hi" => return Some(Box::new(Hindi {})),
            "hy" => return Some(Box::new(Armenian {})),
            "ja" => return Some(Box::new(Japanese {})),
            "ml" => return Some(Box::new(Malayalam {})),
            "mr" => return Some(Box::new(Marathi {})),
            "sk" => return Some(Box::new(Slovak {})),
            "my" => return Some(Box::new(Burmese {})),
            "nl" => return Some(Box::new(Dutch {})),
            "pt" => return Some(Box::new(Portuguese {})),
            "it" => return Some(Box::new(Italian {})),
            "ta" => return Some(Box::new(Tamil {})),
            "te" => return Some(Box::new(Tamil {})),
            "kn" => return Some(Box::new(Kannada {})),
            "kk" => return Some(Box::new(Kazakh {})),
            "pa" => return Some(Box::new(Punjabi {})),
            "pl" => return Some(Box::new(Polish {})),
            "fr" => return Some(Box::new(French {})),
            "fi" => return Some(Box::new(Finnish {})),
            _ => {
                if let Some(fallbacks) = languages::get_fallbacks(current_code) {
                    for next_code in fallbacks {
                        if !visited.contains(next_code) {
                            current_code = next_code;
                            break;
                        }
                    }
                } else {
                    return None; // No fallbacks are found
                }
            }
        }
    }
}

fn chunk_text(text: &str, chunk_size: usize) -> Vec<&str> {
    if chunk_size == 0 || text.len() <= chunk_size {
        return vec![text];
    }

    let mut chunks = Vec::new();

    // Split by paragraph breaks (one or more newlines with optional whitespace)
    // Get paragraph parts and their positions
    let mut paragraphs = Vec::new();
    let mut last_end = 0;

    for mat in PARA_SPLIT_REGEX.find_iter(text) {
        // Add the text before this match
        paragraphs.push((last_end, mat.start()));
        last_end = mat.end();
    }
    // Add the final paragraph
    if last_end < text.len() {
        paragraphs.push((last_end, text.len()));
    }

    if paragraphs.is_empty() {
        return vec![text];
    }

    let mut current_start = 0;
    let mut current_end = 0;
    let mut i = 0;

    while i < paragraphs.len() {
        let (para_start, para_end) = paragraphs[i];
        let para_size = para_end - para_start;

        if para_size > chunk_size {
            if current_end > current_start {
                let safe_end = text.ceil_char_boundary(current_end);
                chunks.push(&text[current_start..safe_end]);
                current_start = 0;
                current_end = 0;
            }

            // Paragraph exceeds chunk_size but has no \n\n boundary to split on.
            // Pass it through whole to avoid splitting mid-sentence or mid-word.
            chunks.push(&text[para_start..para_end]);
            i += 1;
            continue;
        }

        // If this is the first paragraph in the chunk
        if current_end == current_start {
            current_start = para_start;
            current_end = para_end;
            i += 1;
            continue;
        }

        // Check if adding this paragraph would exceed chunk_size
        let potential_size = para_end - current_start;

        if potential_size > chunk_size {
            // Finalize current chunk
            let safe_end = text.ceil_char_boundary(current_end);
            chunks.push(&text[current_start..safe_end]);

            // Start new chunk with current paragraph
            current_start = para_start;
            current_end = para_end;
        } else {
            // Add this paragraph to current chunk
            current_end = para_end;
        }

        i += 1;
    }

    // Add the final chunk if there's remaining content
    if current_end > current_start {
        let safe_end = text.ceil_char_boundary(current_end);
        chunks.push(&text[current_start..safe_end]);
    }

    chunks
}

/// Segments a given text into sentences based on the specified language.
///
/// For texts larger than 10KB, the function chunks the text at paragraph boundaries
/// (`\n\n`) to process large inputs efficiently. Paragraphs that exceed 10KB and
/// contain no internal paragraph breaks are processed as a single unit to avoid
/// splitting mid-sentence or mid-word.
///
/// # Arguments
///
/// * `language_code` - A string slice that holds the language code (e.g., "en" for English, "fr" for French).
/// * `text` - A string slice that holds the text to be segmented.
///
/// # Returns
///
/// A `Vec<&str>` containing the segmented sentences.
///
/// # Example
///
/// ```
/// use sentencex::{languages::English, segment};
///
/// let language_code = "en";
/// let text = "Hello world. This is a test.";
/// let sentences = segment(&English{}, text);
///
/// assert_eq!(sentences, vec!["Hello world. ", "This is a test."]);
/// ```
pub fn segment<'a, L: Language>(language: &L, text: &'a str) -> Vec<&'a str> {
    const CHUNK_SIZE: usize = 10 * 1024; // 10KB

    if text.len() > CHUNK_SIZE {
        let chunks = chunk_text(text, CHUNK_SIZE);
        let mut all_sentences = Vec::new();
        for chunk in chunks {
            let chunk_sentences = language.segment(chunk);
            all_sentences.extend(chunk_sentences);
        }

        all_sentences
    } else {
        language.segment(text)
    }
}

/// Returns detailed sentence boundaries for a given text based on the specified language.
///
/// This function provides low-level access to sentence boundary detection, returning
/// detailed information about each boundary including start/end indices, the text content,
/// boundary symbols, and whether the boundary represents a paragraph break.
///
/// For texts larger than 10KB, the function chunks the text at paragraph boundaries
/// (`\n\n`) to process large inputs efficiently. Paragraphs that exceed 10KB and
/// contain no internal paragraph breaks are processed as a single unit to avoid
/// splitting mid-sentence or mid-word. The returned boundaries maintain correct
/// indices relative to the original text.
///
/// # Arguments
///
/// * `language_code` - A string slice that holds the language code (e.g., "en" for English, "fr" for French).
/// * `text` - A string slice that holds the text to be analyzed.
///
/// # Returns
///
/// A `Vec<SentenceBoundary>` containing detailed information about each sentence boundary.
/// Each `SentenceBoundary` includes:
/// - `start_index`: The character index (Unicode scalar count) where the sentence starts
/// - `end_index`: The character index (Unicode scalar count) where the sentence ends
/// - `text`: A reference to the sentence text (zero-copy)
/// - `boundary_symbol`: The punctuation mark that ended the sentence (if any)
/// - `is_paragraph_break`: Whether this boundary represents a paragraph break ("\n\n")
///
/// # Example
///
/// ```
/// use sentencex::{languages::English, get_sentence_boundaries};
///
/// let text = "Hello world. This is a test.\n\nNew paragraph.";
/// let boundaries = get_sentence_boundaries(&English{}, text);
///
/// for boundary in boundaries {
///     println!("Text: {:?}, Start: {}, End: {}",
///              boundary.text, boundary.start_index, boundary.end_index);
/// }
/// ```
pub fn get_sentence_boundaries<'a, L: Language>(
    language: &L,
    text: &'a str,
) -> Vec<SentenceBoundary<'a>> {
    const CHUNK_SIZE: usize = 10 * 1024; // 10KB

    if text.len() > CHUNK_SIZE {
        let chunks = chunk_text(text, CHUNK_SIZE);
        let mut all_boundaries = Vec::new();
        let mut chunk_offset = 0;

        for chunk in chunks {
            let chunk_boundaries = language.get_sentence_boundaries(chunk);

            // Adjust indices to be relative to original text
            let mut prev_end_index = 0;
            for boundary in chunk_boundaries {
                let start_byte = boundary.start_byte + chunk_offset;
                let end_byte = boundary.end_byte + chunk_offset;

                let start_index = if prev_end_index > 0 {
                    prev_end_index
                } else {
                    text[..start_byte].chars().count()
                };

                let end_index = start_index + boundary.text.chars().count();
                prev_end_index = end_index;

                all_boundaries.push(SentenceBoundary {
                    start_index,
                    end_index,
                    start_byte,
                    end_byte,
                    text: boundary.text,
                    boundary_symbol: boundary.boundary_symbol,
                    is_paragraph_break: boundary.is_paragraph_break,
                });
            }

            chunk_offset += chunk.len();
        }

        all_boundaries
    } else {
        language.get_sentence_boundaries(text)
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    pub fn run_language_tests_for_language<L: Language>(language: &L, test_file: &str) {
        let content = fs::read_to_string(test_file).expect("Failed to read test file");
        let test_cases: Vec<&str> = content.split("===").collect();

        for case in test_cases {
            let case = case.trim();
            if case.is_empty() || case.starts_with('#') {
                continue; // Skip comment and empty lines
            }
            let parts: Vec<&str> = case.split("---")
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .collect();
            if parts.is_empty() {
                continue; // Skip cases with empty lines
            }
            assert_eq!(parts.len(), 2, "Malformed test case: \n{}", case);

            let input = parts[0];
            let expected: Vec<&str> = parts[1].lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect();
            let result = segment(language, input);
            let trimmed_result: Vec<String> =
                result.iter().map(|item| item.trim().to_string()).collect();

            assert_eq!(trimmed_result, expected, "Failed for input: \n{}", input);
        }
    }

    #[test]
    fn test_urdu_segment() {
        run_language_tests_for_language(&language_factory("en").unwrap(), "tests/ur.txt");
    }
    #[test]
    fn test_chinese_segment() {
        run_language_tests_for_language(&language_factory("en").unwrap(), "tests/zh.txt");
    }

    #[test]
    fn test_chunk_text_basic() {
        let text = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        let chunks = chunk_text(text, 20);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], "First paragraph.");
        assert_eq!(chunks[1], "Second paragraph.");
        assert_eq!(chunks[2], "Third paragraph.");
    }

    #[test]
    fn test_chunk_text_no_paragraph_breaks() {
        // Text with no \n\n but longer than chunk_size: returned as a single chunk,
        // never split mid-word or mid-sentence.
        let text =
            "This is a long text without paragraph breaks that should be returned as one chunk.";
        let chunks = chunk_text(text, 20);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn test_segment_no_word_split_at_chunk_boundary() {
        // Reproduce issue #45: text >10KB with no \n\n, where "Christopher" straddles
        // the 10240-byte hard-split boundary (4 bytes "Chri" in chunk 0, "stopher." in chunk 1).
        const CHUNK_SIZE: usize = 10 * 1024;
        // Pad with exactly CHUNK_SIZE - 4 bytes so "Christopher" starts at byte CHUNK_SIZE - 4
        let padding = "a".repeat(CHUNK_SIZE - 4);
        let text = format!("{}Christopher.", padding);
        assert!(text.len() > CHUNK_SIZE, "test input must exceed chunk size");
        // Confirm the split would fall inside "Christopher"
        assert_eq!(&text[CHUNK_SIZE - 4..CHUNK_SIZE], "Chri");

        let sentences = segment("en", &text);
        // "Christopher" must appear whole — no fragment like "Chri" or "stopher." as a sentence
        for s in &sentences {
            assert!(
                !s.trim_end().ends_with("Chri"),
                "Word 'Christopher' was split: sentence ends with 'Chri': {:?}",
                s
            );
            assert!(
                !s.trim_start().starts_with("stopher"),
                "Word 'Christopher' was split: sentence starts with 'stopher': {:?}",
                s
            );
        }
        assert!(
            sentences.iter().any(|s| s.contains("Christopher")),
            "Expected 'Christopher' to appear whole in a sentence, got: {:?}",
            sentences.last()
        );
    }

    #[test]
    fn test_get_sentence_boundaries_no_word_split_at_chunk_boundary() {
        // Same setup as above but via get_sentence_boundaries.
        const CHUNK_SIZE: usize = 10 * 1024;
        let padding = "a".repeat(CHUNK_SIZE - 4);
        let text = format!("{}Christopher.", padding);
        assert!(text.len() > CHUNK_SIZE, "test input must exceed chunk size");

        let boundaries = get_sentence_boundaries("en", &text);

        // No boundary text should be a fragment of "Christopher"
        for b in &boundaries {
            assert!(
                !b.text.trim_end().ends_with("Chri"),
                "Boundary ends with split fragment 'Chri': {:?}",
                b.text
            );
            assert!(
                !b.text.trim_start().starts_with("stopher"),
                "Boundary starts with split fragment 'stopher': {:?}",
                b.text
            );
        }

        // Full text must be reconstructable from boundary spans
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Text reconstruction failed after chunking"
        );
    }

    #[test]
    fn test_segment_automatic_chunking() {
        // Create a text larger than 512KB to trigger chunking
        let small_text = "First sentence. Second sentence.\n\nThird sentence. Fourth sentence.";
        let large_text = small_text.repeat(10000); // This will be > 512KB

        let result = segment(&language_factory("en").unwrap(), &large_text);
        let expected_per_repetition = segment(&language_factory("en").unwrap(), small_text);

        // Verify that we get the expected pattern repeated
        assert!(result.len() >= expected_per_repetition.len() * 9000); // Allow for some variation

        // Test that small text still works normally
        let small_result = segment(&language_factory("en").unwrap(), small_text);
        assert_eq!(small_result, expected_per_repetition);
    }

    #[test]
    fn test_get_sentence_boundaries_with_paragraph_breaks() {
        let text = "Title\n\nSentence 1.\n\nSentence 2.";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        // Should have at least 2 sentences plus paragraph breaks
        assert!(boundaries.len() >= 2);

        // Verify indices are consistent
        for i in 1..boundaries.len() {
            assert!(
                boundaries[i].start_index >= boundaries[i - 1].end_index,
                "Boundary {} starts at {} but previous ends at {}",
                i,
                boundaries[i].start_index,
                boundaries[i - 1].end_index
            );
        }

        // Verify text can be reconstructed
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Reconstructed text doesn't match original"
        );

        // Check that paragraph breaks are detected
        let paragraph_breaks: Vec<_> = boundaries.iter().filter(|b| b.is_paragraph_break).collect();
        assert!(
            paragraph_breaks.len() >= 2,
            "Expected at least 2 paragraph breaks, found {}",
            paragraph_breaks.len()
        );
    }

    #[test]
    fn test_get_sentence_boundaries_with_multibyte_cjk() {
        // Test with Japanese and Chinese characters (multi-byte UTF-8)
        let text = "日本語です。\n\n中文文章。";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        // Should have sentences and paragraph break
        assert!(
            boundaries.len() >= 2,
            "Expected at least 2 boundaries, got {}",
            boundaries.len()
        );

        // Verify indices don't overlap and are monotonically increasing
        for i in 1..boundaries.len() {
            assert!(
                boundaries[i].start_index >= boundaries[i - 1].end_index,
                "Boundary {} starts at {} but previous ends at {}",
                i,
                boundaries[i].start_index,
                boundaries[i - 1].end_index
            );
        }

        // Verify text can be reconstructed (most important for multi-byte UTF-8)
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Reconstructed text doesn't match original.\nOriginal: {:?}\nReconstructed: {:?}",
            text, reconstructed
        );
    }

    #[test]
    fn test_get_sentence_boundaries_with_emoji() {
        // Test with emoji characters (4-byte UTF-8)
        let text = "Hello world 👋.\n\nGoodbye 👋.";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        // Should have sentences and paragraph break
        assert!(
            boundaries.len() >= 2,
            "Expected at least 2 boundaries, got {}",
            boundaries.len()
        );

        // Verify text can be reconstructed (critical for emoji)
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Reconstructed text doesn't match original with emojis"
        );

        // Verify boundary indices are valid
        for boundary in &boundaries {
            assert!(
                boundary.start_index <= boundary.end_index,
                "Invalid boundary: start_index {} > end_index {}",
                boundary.start_index,
                boundary.end_index
            );
        }
    }

    #[test]
    fn test_get_sentence_boundaries_with_mixed_scripts() {
        // Test with mixed ASCII, Latin extended, and CJK
        let text = "English text. Café résumé.\n\n日本語のテキスト。";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        // Verify text reconstruction
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Mixed script text failed reconstruction"
        );

        // Verify indices are properly ordered
        for i in 1..boundaries.len() {
            assert!(
                boundaries[i].start_index >= boundaries[i - 1].end_index,
                "Boundaries not ordered correctly at index {}",
                i
            );
        }

        // Verify all boundaries have valid indices
        for (i, boundary) in boundaries.iter().enumerate() {
            assert!(
                boundary.start_index <= boundary.end_index,
                "Boundary {} has invalid indices: {} > {}",
                i,
                boundary.start_index,
                boundary.end_index
            );
        }
    }

    #[test]
    fn test_get_sentence_boundaries_character_vs_byte_offsets() {
        // This test specifically validates that character indices (start_index/end_index)
        // are correctly distinguished from byte offsets (start_byte/end_byte)
        let text = "Short. 日本語.";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        // Count actual characters in original text
        let total_chars = text.chars().count();
        let total_bytes = text.len();

        // Verify that the character indices sum correctly
        // All boundaries together should cover all characters
        let mut last_char_end = 0;
        for boundary in &boundaries {
            // Each boundary should start where previous ended (no gaps)
            assert_eq!(
                boundary.start_index, last_char_end,
                "Gap in character indices: expected start {}, got {}",
                last_char_end, boundary.start_index
            );
            last_char_end = boundary.end_index;
        }
        assert_eq!(
            last_char_end, total_chars,
            "Character coverage mismatch: ended at {}, total chars = {}",
            last_char_end, total_chars
        );

        // Similarly verify byte offsets
        let mut last_byte_end = 0;
        for boundary in &boundaries {
            assert_eq!(
                boundary.start_byte, last_byte_end,
                "Gap in byte indices: expected start {}, got {}",
                last_byte_end, boundary.start_byte
            );
            last_byte_end = boundary.end_byte;
        }
        assert_eq!(
            last_byte_end, total_bytes,
            "Byte coverage mismatch: ended at {}, total bytes = {}",
            last_byte_end, total_bytes
        );
    }

    #[test]
    fn test_segment_with_multibyte_characters() {
        // Test basic segment function with multi-byte characters
        let text = "日本語です。中文文章。";
        let sentences = segment(&language_factory("en").unwrap(), text);

        // Should segment into sentences
        assert!(sentences.len() > 0, "Should find at least one sentence");

        // Verify reconstruction
        let reconstructed: String = sentences.join("");
        assert_eq!(
            reconstructed, text,
            "Segment reconstruction failed for multi-byte text"
        );
    }

    #[test]
    fn test_segment_quote_followed_by_spaced_dots_does_not_panic() {
        // Minimized from a JPMorgan annual-report TOC line that previously caused
        // quote-extension to push a later boundary backwards.
        let text = "\"x.\" . .";
        let sentences = segment("en", text);

        assert_eq!(sentences, vec![text]);
    }

    #[test]
    fn test_get_sentence_boundaries_quote_followed_by_spaced_dots_does_not_panic() {
        let text = "\"x.\" . .";
        let boundaries = get_sentence_boundaries("en", text);

        assert_eq!(boundaries.len(), 1);
        assert_eq!(boundaries[0].text, text);
        assert_eq!(boundaries[0].start_index, 0);
        assert_eq!(boundaries[0].end_index, text.chars().count());
        assert_eq!(boundaries[0].start_byte, 0);
        assert_eq!(boundaries[0].end_byte, text.len());
        assert_eq!(boundaries[0].boundary_symbol.as_deref(), Some("."));
        assert!(!boundaries[0].is_paragraph_break);
    }

    #[test]
    fn test_boundary_symbol_detection_with_trailing_space() {
        // This test reproduces the bug from issue #35:
        // boundary_symbols are not detected if space follows boundary_symbol
        let text = "Hello world. This is a test.Another test. And another test.";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        // Verify we have at least 4 boundaries (4 sentences)
        assert!(
            boundaries.len() >= 4,
            "Expected at least 4 boundaries, got {}",
            boundaries.len()
        );

        // Check each boundary has the correct boundary_symbol
        let non_paragraph: Vec<_> = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect();

        // All 4 sentences should have period as boundary symbol
        for (i, boundary) in non_paragraph.iter().enumerate() {
            assert!(
                boundary.boundary_symbol.is_some(),
                "Boundary {} should have a boundary_symbol, got None",
                i
            );
            assert_eq!(
                boundary.boundary_symbol.as_deref(),
                Some("."),
                "Boundary {} should have period as symbol",
                i
            );
        }
    }

    #[test]
    fn test_boundary_symbol_with_multiple_trailing_spaces() {
        // Test that multiple trailing spaces don't prevent symbol detection
        let text = "Hello.  This is another.   Yet another.";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        let non_paragraph = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect::<Vec<_>>();

        // All sentence boundaries should have period symbol
        for (i, boundary) in non_paragraph.iter().enumerate() {
            assert_eq!(
                boundary.boundary_symbol.as_deref(),
                Some("."),
                "Boundary {} should have period symbol despite trailing spaces",
                i
            );
        }
    }

    #[test]
    fn test_boundary_symbol_with_mixed_terminators() {
        // Test various sentence terminators with trailing spaces
        let text = "Hello! How are you? I'm fine. Yes.";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        let non_paragraph = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect::<Vec<_>>();

        // Verify we detect the different terminators
        let symbols: Vec<_> = non_paragraph
            .iter()
            .filter_map(|b| b.boundary_symbol.as_deref())
            .collect();

        assert!(
            symbols.iter().any(|&s| s == "!"),
            "Should detect exclamation mark"
        );
        assert!(
            symbols.iter().any(|&s| s == "?"),
            "Should detect question mark"
        );
        assert!(symbols.iter().any(|&s| s == "."), "Should detect period");
    }

    #[test]
    fn test_boundary_symbol_with_cjk_terminator() {
        // Test with CJK full stop (。) which is a valid sentence terminator
        let text = "日本語です。中文です。";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        let non_paragraph = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect::<Vec<_>>();

        // Should detect CJK terminators
        let with_cjk_stop = non_paragraph
            .iter()
            .filter(|b| b.boundary_symbol.as_deref() == Some("。"))
            .count();

        assert!(
            with_cjk_stop >= 1,
            "Should detect at least one CJK full stop, got {}",
            with_cjk_stop
        );
    }

    #[test]
    fn test_boundary_symbol_with_tabs_and_spaces() {
        // Test with mixed whitespace (tabs and spaces)
        let text = "First sentence.\t\n  Second sentence. Third one!";
        let boundaries = get_sentence_boundaries(&language_factory("en").unwrap(), text);

        let non_paragraph = boundaries
            .iter()
            .filter(|b| !b.is_paragraph_break)
            .collect::<Vec<_>>();

        // All should have boundary symbols
        for (i, boundary) in non_paragraph.iter().enumerate() {
            assert!(
                boundary.boundary_symbol.is_some(),
                "Boundary {} should have boundary_symbol despite mixed whitespace",
                i
            );
        }

        // Verify reconstruction still works
        let reconstructed: String = boundaries.iter().map(|b| b.text).collect();
        assert_eq!(
            reconstructed, text,
            "Text reconstruction failed with mixed whitespace"
        );
    }
}
