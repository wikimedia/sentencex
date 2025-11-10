#[cfg(feature = "html")]
mod html_integration_tests {
    use sentencex::html::{HtmlSegmentConfig, segment_html};

    #[test]
    fn test_basic_paragraph_segmentation() {
        let html = "<p>Hello world. This is a test sentence.</p>";
        let config = HtmlSegmentConfig::default();

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);
        assert!(result.sentences[0].text.trim().starts_with("Hello world"));
        assert!(
            result.sentences[1]
                .text
                .trim()
                .starts_with("This is a test")
        );
        assert!(result.marked_html.is_some());
    }

    #[test]
    fn test_cross_tag_sentence() {
        let html = "<p>This is a <em>bold statement</em> that continues here. This is another sentence.</p>";
        let config = HtmlSegmentConfig::default();

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);

        // First sentence spans multiple nodes
        assert!(result.sentences[0].html_ranges.len() >= 2);
        assert!(result.sentences[0].text.contains("bold statement"));
        assert!(result.sentences[0].text.contains("continues here"));
    }

    #[test]
    fn test_nested_inline_elements() {
        let html = "<p>The <strong>quick <em>brown</em> fox</strong> jumps. Over the lazy dog.</p>";
        let config = HtmlSegmentConfig::default();

        let result = segment_html(html, &config).unwrap();

        // Debug output
        println!("Found {} sentences:", result.sentences.len());
        for (i, sentence) in result.sentences.iter().enumerate() {
            println!("Sentence {}: '{}'", i, sentence.text);
        }

        assert_eq!(result.sentences.len(), 2);
        let sentence0_contains_fox = result.sentences[0].text.contains("quick")
            && result.sentences[0].text.contains("brown")
            && result.sentences[0].text.contains("fox")
            && result.sentences[0].text.contains("jumps");
        assert!(
            sentence0_contains_fox,
            "First sentence should contain 'quick brown fox jumps', got: '{}'",
            result.sentences[0].text
        );
        assert!(result.sentences[1].text.contains("Over the lazy dog"));
    }

    #[test]
    fn test_excluded_elements() {
        let html = r#"
            <p>This is normal text. It has sentences.</p>
            <script>
                console.log("This should not be segmented. Even with periods.");
            </script>
            <style>
                body { font-family: serif; }
                /* This also should not be segmented. */
            </style>
            <pre>
                Code block with sentences. Should be excluded.
            </pre>
            <p>More normal text here. Final sentence.</p>
        "#;
        let config = HtmlSegmentConfig::default();

        let result = segment_html(html, &config).unwrap();

        // Debug output
        println!("Found {} sentences:", result.sentences.len());
        for (i, sentence) in result.sentences.iter().enumerate() {
            println!("Sentence {}: '{}'", i, sentence.text.trim());
        }

        // Should only find sentences from <p> elements
        assert_eq!(result.sentences.len(), 4);

        // Verify no script/style/pre content
        for sentence in &result.sentences {
            assert!(!sentence.text.contains("console.log"));
            assert!(!sentence.text.contains("font-family"));
            assert!(!sentence.text.contains("Code block"));
        }

        // Verify we got the expected content
        assert!(
            result
                .sentences
                .iter()
                .any(|s| s.text.contains("normal text"))
        );
        assert!(
            result
                .sentences
                .iter()
                .any(|s| s.text.contains("Final sentence"))
        );
    }

    #[test]
    fn test_multiple_paragraphs() {
        let html = r#"
            <div>
                <p>First paragraph sentence one. First paragraph sentence two.</p>
                <p>Second paragraph here. Another sentence in second paragraph.</p>
            </div>
        "#;
        let config = HtmlSegmentConfig::default();

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 4);

        // Check sentence distribution
        let texts: Vec<_> = result.sentences.iter().map(|s| s.text.trim()).collect();
        assert!(
            texts
                .iter()
                .any(|&t| t.contains("First paragraph sentence one"))
        );
        assert!(
            texts
                .iter()
                .any(|&t| t.contains("First paragraph sentence two"))
        );
        assert!(texts.iter().any(|&t| t.contains("Second paragraph here")));
        assert!(
            texts
                .iter()
                .any(|&t| t.contains("Another sentence in second"))
        );
    }

    #[test]
    fn test_marked_html_structure() {
        let html = "<p>Hello world. This is a test.</p>";
        let config = HtmlSegmentConfig {
            add_marks: true,
            ..Default::default()
        };

        let result = segment_html(html, &config).unwrap();
        let marked = result.marked_html.unwrap();

        // Should contain mark tags with sentence IDs
        assert!(marked.contains(r#"<mark data-sent="0""#));
        assert!(marked.contains(r#"<mark data-sent="1""#));

        // Should preserve original text
        assert!(marked.contains("Hello world"));
        assert!(marked.contains("This is a test"));

        // Should maintain HTML structure
        assert!(marked.contains("<p>"));
        assert!(marked.contains("</p>"));
    }

    #[test]
    fn test_complex_real_world_example() {
        let html = concat!(
            "<h1>Article Title</h1>",
            "<p>This is the <strong>introduction</strong> paragraph. It sets up the topic nicely.</p>",
            "<p>The second paragraph has <a href=\"#\">a link</a> and continues with more text.</p>",
            "<blockquote>",
            "<p>This is a quoted sentence. It should be segmented properly.</p>",
            "<cite>- Some Author</cite>",
            "</blockquote>",
            "<p>Final paragraph with <em>emphasis</em> and <code>inline code</code>. Last sentence here.</p>",
        );
        let config = HtmlSegmentConfig::default();

        let result = segment_html(html, &config).unwrap();

        // Should find several sentences
        // assert!(result.sentences.len() >= 6);
        dbg!(&result.sentences);

        // Check for expected content
        let all_text: String = result.sentences.iter().map(|s| s.text.as_str()).collect();
        assert!(all_text.contains("Article Title"));
        assert!(all_text.contains("introduction"));
        assert!(all_text.contains("quoted sentence"));
        assert!(all_text.contains("inline code"));
        assert!(all_text.contains("Last sentence"));

        // Verify marked HTML is valid and contains marks
        if let Some(marked) = &result.marked_html {
            assert!(marked.contains(r#"<mark data-sent="#));
            assert!(marked.contains("</mark>"));

            // Should preserve all original structure
            assert!(marked.contains("<article>"));
            assert!(marked.contains("<h1>"));
            assert!(marked.contains("<blockquote>"));
            assert!(marked.contains("<cite>"));
        }
    }

    #[test]
    fn test_partial_content_across_boundaries() {
        let html = "<p>The <strong>quick brown fox jumps. Over</strong> the lazy dog.</p>";
        let config = HtmlSegmentConfig::default();

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);

        // First sentence should end at "jumps."
        assert!(result.sentences[0].text.contains("jumps"));
        assert!(!result.sentences[0].text.contains("Over"));

        // Second sentence should start with "Over"
        assert!(result.sentences[1].text.contains("Over"));
        assert!(result.sentences[1].text.contains("lazy dog"));
    }

    #[test]
    fn test_whitespace_handling() {
        let html = "<p>Sentence one.    Sentence two with extra spaces.</p>";
        let config = HtmlSegmentConfig::default();

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);

        // Should preserve whitespace in the original ranges
        assert!(result.sentences[0].text.contains("Sentence one."));
        assert!(result.sentences[1].text.contains("Sentence two"));
    }

    #[test]
    fn test_empty_and_whitespace_nodes() {
        let html = r#"
            <p>
                Text here.
                
                More text.
            </p>
        "#;
        let config = HtmlSegmentConfig::default();

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);
        assert!(result.sentences[0].text.trim() == "Text here.");
        assert!(result.sentences[1].text.trim() == "More text.");
    }

    #[test]
    fn test_no_marking_option() {
        let html = "<p>Hello world. This is a test.</p>";
        let config = HtmlSegmentConfig {
            add_marks: false,
            ..Default::default()
        };

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);
        assert!(result.marked_html.is_none());
    }

    #[test]
    fn test_different_languages() {
        let html = "<p>Bonjour le monde. Ceci est un test.</p>";
        let config = HtmlSegmentConfig {
            language_code: "fr".to_string(),
            add_marks: false,
            ..Default::default()
        };

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);
        assert!(result.sentences[0].text.contains("Bonjour"));
        assert!(result.sentences[1].text.contains("Ceci est"));
    }

    #[test]
    fn test_custom_exclusions() {
        let html = r#"
            <p>Normal text here.</p>
            <aside>Sidebar content with sentences. Should be excluded.</aside>
            <p>More normal text.</p>
        "#;

        let mut config = HtmlSegmentConfig::default();
        config.exclude_elements.push("aside".to_string());
        config.add_marks = false;

        let result = segment_html(html, &config).unwrap();

        // Should only find sentences from <p> elements
        assert_eq!(result.sentences.len(), 2);

        // Verify no aside content
        for sentence in &result.sentences {
            assert!(!sentence.text.contains("Sidebar"));
            assert!(!sentence.text.contains("excluded"));
        }
    }
}
