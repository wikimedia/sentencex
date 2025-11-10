#[cfg(feature = "html")]
#[test]
fn debug_exclusion() {
    use sentencex::html::{segment_html, HtmlSegmentConfig};

    let html = r#"<p>Test.</p><pre>Code.</pre>"#;
    let config = HtmlSegmentConfig::default();
    
    let result = segment_html(html, &config).unwrap();
    
    println!("Sentences found:");
    for (i, sentence) in result.sentences.iter().enumerate() {
        println!("  {}: '{}'", i, sentence.text.trim());
    }
    
    assert_eq!(result.sentences.len(), 1);
    assert!(result.sentences[0].text.contains("Test"));
    assert!(!result.sentences.iter().any(|s| s.text.contains("Code")));
}
