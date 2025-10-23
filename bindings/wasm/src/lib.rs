use ::sentencex::{get_sentence_boundaries as _get_sentence_boundaries, segment as _segment};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

/// Segments a given text into sentences based on the specified language.
///
/// # Arguments
///
/// * `language` - A string slice that holds the language code (e.g., "en" for English, "fr" for French).
/// * `text` - A string slice that holds the text to be segmented.
///
/// # Returns
///
/// A `JsValue` containing the segmented sentences as a JavaScript array.
///
/// # Example
///
/// ```javascript
/// import init, { segment } from './pkg/sentencex_wasm.js';
///
/// async function run() {
///     await init();
///     const sentences = segment("en", "Hello world. This is a test.");
///     console.log(sentences); // ["Hello world. ", "This is a test."]
/// }
/// run();
/// ```
#[wasm_bindgen]
pub fn segment(language: &str, text: &str) -> JsValue {
    let sentences = _segment(language, text);
    serde_wasm_bindgen::to_value(&sentences).expect("")
}

/// Returns detailed sentence boundaries for a given text based on the specified language.
///
/// # Arguments
///
/// * `language` - A string slice that holds the language code (e.g., "en" for English, "fr" for French).
/// * `text` - A string slice that holds the text to be analyzed.
///
/// # Returns
///
/// A `JsValue` containing an array of sentence boundary objects. Each object contains:
/// - `start_index`: The byte index where the sentence starts
/// - `end_index`: The byte index where the sentence ends
/// - `text`: The sentence text
/// - `boundary_symbol`: The punctuation mark that ended the sentence (if any)
/// - `is_paragraph_break`: Whether this boundary represents a paragraph break
///
/// # Example
///
/// ```javascript
/// import init, { get_sentence_boundaries } from './pkg/sentencex_wasm.js';
///
/// async function run() {
///     await init();
///     const boundaries = get_sentence_boundaries("en", "Hello world. This is a test.");
///     console.log(boundaries); // Array of boundary objects
/// }
/// run();
/// ```
#[wasm_bindgen]
pub fn get_sentence_boundaries(language: &str, text: &str) -> JsValue {
    let boundaries = _get_sentence_boundaries(language, text);
    serde_wasm_bindgen::to_value(&boundaries).expect("")
}
