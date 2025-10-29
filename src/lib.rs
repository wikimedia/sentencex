use languages::{
    Amharic, Arabic, Armenian, Bengali, Bulgarian, Burmese, Catalan, Danish, Deutch, Dutch,
    English, Finnish, French, Greek, Gujarati, Hindi, Italian, Japanese, Kannada, Kazakh, Language,
    Malayalam, Marathi, Polish, Portuguese, Punjabi, Slovak, Spanish, Tamil,
};
use serde::Serialize;
use std::io::Read;

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

/// Segments a given text into sentences based on the specified language.
///
/// # Arguments
///
/// * `language_code` - A string slice that holds the language code (e.g., "en" for English, "fr" for French).
/// * `text` - A string slice that holds the text to be segmented.
///
/// # Returns
///
/// A `Vec<String>` containing the segmented sentences.
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
pub fn segment(language_code: &str, text: &str) -> Vec<String> {
    let language = language_factory(language_code);
    language.segment(text)
}

/// Segments a given text into sentences based on the specified language, returning borrowed slices.
///
/// This is a zero-copy variant that returns references to the original text instead of allocating
/// new strings. This is significantly more efficient for large texts.
///
/// # Arguments
///
/// * `language_code` - A string slice that holds the language code (e.g., "en" for English, "fr" for French).
/// * `text` - A string slice that holds the text to be segmented.
///
/// # Returns
///
/// A `Vec<&str>` containing references to the segmented sentences.
///
/// # Example
///
/// ```
/// use sentencex::segment_borrowed;
///
/// let language_code = "en";
/// let text = "Hello world. This is a test.";
/// let sentences = segment_borrowed(language_code, text);
///
/// assert_eq!(sentences, vec!["Hello world. ", "This is a test."]);
/// ```
pub fn segment_borrowed<'a>(language_code: &str, text: &'a str) -> Vec<&'a str> {
    let language = language_factory(language_code);
    language.segment_borrowed(text)
}

/// Returns detailed sentence boundaries for a given text based on the specified language.
///
/// This function provides low-level access to sentence boundary detection, returning
/// detailed information about each boundary including start/end indices, the text content,
/// boundary symbols, and whether the boundary represents a paragraph break.
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
    let language = language_factory(language_code);
    language.get_sentence_boundaries(text)
}

/// Segments text from a reader in chunks for efficient processing of large files.
///
/// This function is designed for processing very large files (>10MB) that might not fit
/// comfortably in memory. It reads the input in chunks and processes them incrementally.
///
/// # Arguments
///
/// * `language_code` - A string slice that holds the language code (e.g., "en" for English, "fr" for French).
/// * `reader` - Any type implementing `Read` (e.g., `File`, `&[u8]`, etc.)
/// * `chunk_size` - Size of chunks to process at once (recommended: 64KB - 1MB)
///
/// # Returns
///
/// A `Result<Vec<String>>` containing the segmented sentences or an IO error.
///
/// # Example
///
/// ```no_run
/// use std::fs::File;
/// use sentencex::segment_chunked;
///
/// let file = File::open("large_document.txt").unwrap();
/// let sentences = segment_chunked("en", file, 65536).unwrap();
/// println!("Found {} sentences", sentences.len());
/// ```
pub fn segment_chunked<R: Read>(
    language_code: &str,
    mut reader: R,
    chunk_size: usize,
) -> std::io::Result<Vec<String>> {
    let language = language_factory(language_code);
    let mut sentences = Vec::new();
    let mut buffer = vec![0u8; chunk_size];
    let mut leftover = String::new();

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            // Process any remaining text
            if !leftover.is_empty() {
                let final_sentences = language.segment(&leftover);
                sentences.extend(final_sentences);
            }
            break;
        }

        // Combine leftover with new chunk
        let mut chunk_text = leftover.clone();
        chunk_text.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));

        // Find last sentence boundary to avoid cutting mid-sentence
        // Look for last newline or period followed by space
        let split_pos = if bytes_read == chunk_size {
            // Not at EOF, find a good split point
            chunk_text.rfind(|c| c == '\n' || c == '.')
                .map(|pos| pos + 1)
                .unwrap_or(chunk_text.len())
        } else {
            // At EOF, process everything
            chunk_text.len()
        };

        let (to_process, remainder) = chunk_text.split_at(split_pos);
        
        if !to_process.is_empty() {
            let chunk_sentences = language.segment(to_process);
            sentences.extend(chunk_sentences);
        }
        
        leftover = remainder.to_string();
    }

    Ok(sentences)
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
}
