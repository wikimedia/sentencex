use wasm_bindgen::prelude::*;
use ::sentencex::segment as _segment;

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
    JsValue::from_serde(&sentences).expect("Failed to serialize sentences")
}
