use languages::{
    Amharic, Arabic, Armenian, Bengali, Bulgarian, Burmese, Catalan, Danish, Deutch, Dutch,
    English, Finnish, French, Greek, Gujarati, Hindi, Italian, Japanese, Kannada, Kazakh, Language,
    Malayalam, Marathi, Polish, Portuguese, Punjabi, Slovak, Spanish, Tamil,
};
use regex::Regex;
use serde::Serialize;

mod constants;
pub mod languages;

use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref LANGUAGE_FALLBACKS: HashMap<&'static str, Vec<&'static str>> = {
        let yaml_data = include_str!("./languages/fallbacks.yaml");
        serde_yaml::from_str(yaml_data).expect("Failed to parse fallbacks.yaml")
    };
}

#[derive(Debug, Clone, Serialize)]
pub struct SentenceBoundary<'a> {
    pub start_index: usize,
    pub end_index: usize,
    pub text: &'a str,
    pub boundary_symbol: Option<String>,
    pub is_paragraph_break: bool,
}

pub fn language_factory(language_code: &str) -> Box<dyn Language> {
    let mut current_code = language_code;
    let mut visited = std::collections::HashSet::new();

    loop {
        if visited.contains(current_code) {
            current_code = "en"; // Default to English if a cycle is detected
        } else {
            visited.insert(current_code);
        }

        match current_code {
            "am" => return Box::new(Amharic {}),
            "ar" => return Box::new(Arabic {}),
            "bg" => return Box::new(Bulgarian {}),
            "bn" => return Box::new(Bengali {}),
            "ca" => return Box::new(Catalan {}),
            "da" => return Box::new(Danish {}),
            "de" => return Box::new(Deutch {}),
            "en" => return Box::new(English {}),
            "es" => return Box::new(Spanish {}),
            "el" => return Box::new(Greek {}),
            "gu" => return Box::new(Gujarati {}),
            "hi" => return Box::new(Hindi {}),
            "hy" => return Box::new(Armenian {}),
            "ja" => return Box::new(Japanese {}),
            "ml" => return Box::new(Malayalam {}),
            "mr" => return Box::new(Marathi {}),
            "sk" => return Box::new(Slovak {}),
            "my" => return Box::new(Burmese {}),
            "nl" => return Box::new(Dutch {}),
            "pt" => return Box::new(Portuguese {}),
            "it" => return Box::new(Italian {}),
            "ta" => return Box::new(Tamil {}),
            "te" => return Box::new(Tamil {}),
            "kn" => return Box::new(Kannada {}),
            "kk" => return Box::new(Kazakh {}),
            "pa" => return Box::new(Punjabi {}),
            "pl" => return Box::new(Polish {}),
            "fr" => return Box::new(French {}),
            "fi" => return Box::new(Finnish {}),
            _ => {
                if let Some(fallbacks) = LANGUAGE_FALLBACKS.get(current_code) {
                    for next_code in fallbacks {
                        if !visited.contains(next_code) {
                            current_code = next_code;
                            break;
                        }
                    }
                } else {
                    current_code = "en"; // Default to English if no fallbacks are found
                }
            }
        }
    }
}

/// Find the nearest valid UTF-8 character boundary at or before the given byte index
fn find_char_boundary(text: &str, mut byte_index: usize) -> usize {
    // If we're already at or past the end, return text length
    if byte_index >= text.len() {
        return text.len();
    }

    // Walk forwards until we find a valid character boundary
    while byte_index < text.len() && !text.is_char_boundary(byte_index) {
        byte_index += 1;
    }

    byte_index
}

fn chunk_text(text: &str, chunk_size: usize) -> Vec<&str> {
    if chunk_size == 0 || text.len() <= chunk_size {
        return vec![text];
    }

    let mut chunks = Vec::new();

    // Split by paragraph breaks (one or more newlines with optional whitespace)
    let re = Regex::new(r"\n[\r\s]*\n").unwrap();

    // Get paragraph parts and their positions
    let mut paragraphs = Vec::new();
    let mut last_end = 0;

    for mat in re.find_iter(text) {
        // Add the text before this match
        paragraphs.push((last_end, mat.start()));
        last_end = mat.end();
    }
    // Add the final paragraph
    if last_end < text.len() {
        paragraphs.push((last_end, text.len()));
    }

    if paragraphs.is_empty() {
        eprintln!("No para breaks?");
        return vec![text];
    } else {
        eprintln!("Found {:} paragraphs", paragraphs.len());
    }

    let mut current_start = 0;
    let mut current_end = 0;
    let mut i = 0;

    while i < paragraphs.len() {
        let (para_start, para_end) = paragraphs[i];

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
            let safe_end = find_char_boundary(text, current_end);
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
    if current_start < text.len() {
        let safe_end = find_char_boundary(text, current_end);
        chunks.push(&text[current_start..safe_end]);
    }

    chunks
}

/// Segments a given text into sentences based on the specified language.
///
/// For texts larger than CHUNK_SIZE, the function automatically chunks the text at paragraph
/// boundaries (double newlines) to handle large inputs efficiently. The only fallback
/// boundary is end of file.
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
/// use sentencex::segment;
///
/// let language_code = "en";
/// let text = "Hello world. This is a test.";
/// let sentences = segment(language_code, text);
///
/// assert_eq!(sentences, vec!["Hello world. ", "This is a test."]);
/// ```
pub fn segment<'a>(language_code: &str, text: &'a str) -> Vec<&'a str> {
    const CHUNK_SIZE: usize = 10 * 1024; // 10KB

    let language = language_factory(language_code);

    if text.len() > CHUNK_SIZE {
        let chunks = chunk_text(text, CHUNK_SIZE);
        let mut all_sentences = Vec::new();
        eprintln!("Processing {:?} chunks", chunks.len());
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
/// For texts larger than chunk_size, the function automatically chunks the text at paragraph
/// boundaries (double newlines) to handle large inputs efficiently. The returned boundaries
/// maintain correct indices relative to the original text.
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
/// - `start_index`: The byte index where the sentence starts
/// - `end_index`: The byte index where the sentence ends
/// - `text`: A reference to the sentence text (zero-copy)
/// - `boundary_symbol`: The punctuation mark that ended the sentence (if any)
/// - `is_paragraph_break`: Whether this boundary represents a paragraph break ("\n\n")
///
/// # Example
///
/// ```
/// use sentencex::get_sentence_boundaries;
///
/// let language_code = "en";
/// let text = "Hello world. This is a test.\n\nNew paragraph.";
/// let boundaries = get_sentence_boundaries(language_code, text);
///
/// for boundary in boundaries {
///     println!("Text: {:?}, Start: {}, End: {}",
///              boundary.text, boundary.start_index, boundary.end_index);
/// }
/// ```
pub fn get_sentence_boundaries<'a>(
    language_code: &str,
    text: &'a str,
) -> Vec<SentenceBoundary<'a>> {
    const CHUNK_SIZE: usize = 10 * 100; // 10KB

    let language = language_factory(language_code);

    if text.len() > CHUNK_SIZE {
        let chunks = chunk_text(text, CHUNK_SIZE);
        let mut all_boundaries = Vec::new();
        let mut chunk_offset = 0;

        for chunk in chunks {
            let chunk_boundaries = language.get_sentence_boundaries(chunk);

            // Adjust indices to be relative to original text
            for boundary in chunk_boundaries {
                all_boundaries.push(SentenceBoundary {
                    start_index: boundary.start_index + chunk_offset,
                    end_index: boundary.end_index + chunk_offset,
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

    pub fn run_language_tests_for_language(language: &str, test_file: &str) {
        let content = fs::read_to_string(test_file).expect("Failed to read test file");
        let test_cases: Vec<&str> = content.split("===\n").collect();

        for case in test_cases {
            if case.trim().starts_with('#') {
                continue; // Skip comment lines
            }
            let parts: Vec<&str> = case.split("---\n").collect();
            if parts.len() != 2 {
                continue; // Skip malformed test cases
            }

            let input = parts[0].trim();
            let expected: Vec<&str> = parts[1].lines().map(|line| line.trim()).collect();
            let result = segment(language, input);
            let trimmed_result: Vec<String> =
                result.iter().map(|item| item.trim().to_string()).collect();

            assert_eq!(trimmed_result, expected, "Failed for input: \n{}", input);
        }
    }

    #[test]
    fn test_urdu_segment() {
        run_language_tests_for_language("ur", "tests/ur.txt");
    }
    #[test]
    fn test_chinese_segment() {
        run_language_tests_for_language("zh", "tests/zh.txt");
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
        let text =
            "This is a long text without paragraph breaks that should be returned as one chunk.";
        let chunks = chunk_text(text, 20);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], text);
    }

    #[test]
    fn test_segment_automatic_chunking() {
        // Create a text larger than 512KB to trigger chunking
        let small_text = "First sentence. Second sentence.\n\nThird sentence. Fourth sentence.";
        let large_text = small_text.repeat(10000); // This will be > 512KB

        let result = segment("en", &large_text);
        let expected_per_repetition = segment("en", small_text);

        // Verify that we get the expected pattern repeated
        assert!(result.len() >= expected_per_repetition.len() * 9000); // Allow for some variation

        // Test that small text still works normally
        let small_result = segment("en", small_text);
        assert_eq!(small_result, expected_per_repetition);
    }
}
