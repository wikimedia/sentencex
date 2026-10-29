use sentencex::{get_sentence_boundaries, segment};

const LANGUAGES: [&str; 5] = ["en", "de", "sv", "fi", "zh"];
const INPUTS: [&str; 6] = [
    "S\u{0085}.",
    "x S\u{0085}.",
    "S \u{0085}.",
    "S\u{0085}\u{0085}.",
    "S\u{0085}. Next.",
    "Fig. S\u{0085}. Next.",
];

// Regression tests for https://github.com/wikimedia/sentencex/issues/78.
#[test]
fn test_segment_single_letter_before_multibyte_whitespace_and_period() {
    for language in LANGUAGES {
        for text in INPUTS {
            let sentences = segment(language, text);
            assert_eq!(
                sentences.join(""),
                text,
                "language: {language}, input: {text:?}"
            );
        }
    }
}

#[test]
fn test_get_sentence_boundaries_single_letter_before_multibyte_whitespace_and_period() {
    for language in LANGUAGES {
        for text in INPUTS {
            let boundaries = get_sentence_boundaries(language, text);
            let reconstructed: String = boundaries.iter().map(|boundary| boundary.text).collect();
            assert_eq!(reconstructed, text, "language: {language}, input: {text:?}");

            for boundary in boundaries {
                assert_eq!(
                    &text[boundary.start_byte..boundary.end_byte],
                    boundary.text,
                    "language: {language}, input: {text:?}"
                );
            }
        }
    }
}
