use sentencex::{get_sentence_boundaries, language_factory, segment};

const LANGUAGES: [&str; 5] = ["en", "de", "sv", "fi", "zh"];
const WHITESPACE: [&str; 6] = [
    "\u{0085}", "\u{00a0}", "\u{2003}", "\u{2028}", "\u{2029}", "\u{3000}",
];
const INPUTS: [&str; 6] = [
    "S\u{0085}.",
    "x S\u{0085}.",
    "S \u{0085}.",
    "S\u{0085}\u{0085}.",
    "S\u{0085}. Next.",
    "Fig. S\u{0085}. Next.",
];

fn inputs() -> impl Iterator<Item = String> {
    INPUTS.into_iter().flat_map(|input| {
        WHITESPACE
            .into_iter()
            .map(move |whitespace| input.replace('\u{0085}', whitespace))
    })
}

// Regression tests for https://github.com/wikimedia/sentencex/issues/78.
#[test]
fn test_segment_single_letter_before_multibyte_whitespace_and_period() {
    for language in LANGUAGES {
        for text in inputs() {
            let sentences = segment(language, &text);
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
        for text in inputs() {
            let boundaries = get_sentence_boundaries(language, &text);
            let reconstructed: String = boundaries.iter().map(|boundary| boundary.text).collect();
            assert_eq!(reconstructed, text, "language: {language}, input: {text:?}");

            for boundary in boundaries {
                assert_eq!(
                    &text[boundary.start_byte..boundary.end_byte],
                    boundary.text,
                    "language: {language}, input: {text:?}"
                );
                assert_eq!(
                    boundary.start_index,
                    text[..boundary.start_byte].chars().count()
                );
                assert_eq!(
                    boundary.end_index,
                    text[..boundary.end_byte].chars().count()
                );
            }
        }
    }
}

#[test]
fn test_name_initial_with_trailing_whitespace() {
    let language = language_factory("en");

    for whitespace in ["", " ", "\t", " \u{0085}\t"].into_iter().chain(WHITESPACE) {
        for (prefix, expected) in [
            ("", false),
            ("x ", false),
            ("Fig. ", true),
            ("George ", true),
        ] {
            let head = format!("{prefix}S{whitespace}");
            assert_eq!(
                language.is_name_initial(&head, " Next."),
                expected,
                "head: {head:?}"
            );
            assert!(
                language.is_name_initial(&head, " R. Tolkien"),
                "head: {head:?}"
            );
        }
    }
}
