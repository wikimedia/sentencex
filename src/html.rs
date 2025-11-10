//! HTML sentence segmentation
//!
//! This module provides functionality to segment sentences within HTML content
//! while preserving the HTML structure and optionally adding `<mark>` tags
//! around sentence boundaries.

use serde::Serialize;
use std::collections::HashMap;
use tree_sitter::{Node, Parser};

use crate::{SentenceBoundary, get_sentence_boundaries};

// Minimal HtmlTag and helpers adapted from reference implementation
#[derive(Debug, Clone)]
struct HtmlTag {
    name: String,
    attributes: HashMap<String, String>,
    start_byte: usize,
    end_byte: usize,
}

impl HtmlTag {
    fn new(name: String, start_byte: usize, end_byte: usize) -> Self {
        Self {
            name,
            attributes: HashMap::new(),
            start_byte,
            end_byte,
        }
    }
}

fn parse_attribute(attr_node: &Node, source: &[u8]) -> Option<(String, String)> {
    let mut attr_name: Option<String> = None;
    let mut attr_value = String::new();

    for child in attr_node.children(&mut attr_node.walk()) {
        match child.kind() {
            "attribute_name" => {
                attr_name = child.utf8_text(source).ok().map(|s| s.to_string());
            }
            "quoted_attribute_value" => {
                for grandchild in child.children(&mut child.walk()) {
                    if grandchild.kind() == "attribute_value" {
                        if let Ok(value) = grandchild.utf8_text(source) {
                            attr_value = value.to_string();
                        }
                    }
                }
            }
            "attribute_value" => {
                if let Ok(value) = child.utf8_text(source) {
                    attr_value = value.to_string();
                }
            }
            _ => {}
        }
    }

    attr_name.map(|name| (name, attr_value))
}

fn parse_element(element_node: &Node, source: &[u8]) -> Option<HtmlTag> {
    // Handle special element node kinds like script_element/style_element
    match element_node.kind() {
        "script_element" => {
            return Some(HtmlTag::new(
                "script".to_string(),
                element_node.start_byte(),
                element_node.end_byte(),
            ));
        }
        "style_element" => {
            return Some(HtmlTag::new(
                "style".to_string(),
                element_node.start_byte(),
                element_node.end_byte(),
            ));
        }
        _ => {}
    }

    // Find the start_tag child by iterating children (field names vary across versions)
    let start_tag = element_node
        .children(&mut element_node.walk())
        .find(|child| child.kind() == "start_tag")?;

    // Extract tag_name by iterating children of start_tag
    let tag_name_node = start_tag
        .children(&mut start_tag.walk())
        .find(|child| child.kind() == "tag_name")?;

    let tag_name_str = tag_name_node.utf8_text(source).ok()?.to_ascii_lowercase();

    let mut html_tag = HtmlTag::new(
        tag_name_str,
        element_node.start_byte(),
        element_node.end_byte(),
    );

    // Parse attributes
    for child in start_tag.children(&mut start_tag.walk()) {
        if child.kind() == "attribute" {
            if let Some((attr_name, attr_value)) = parse_attribute(&child, source) {
                html_tag.attributes.insert(attr_name, attr_value);
            }
        }
    }

    Some(html_tag)
}

/// Configuration for HTML sentence segmentation
#[derive(Debug, Clone)]
pub struct HtmlSegmentConfig {
    /// Language code for sentence segmentation (e.g., "en", "fr")
    pub language_code: String,
    /// Whether to add `<mark>` tags around sentences
    pub add_marks: bool,
    /// HTML elements to exclude from segmentation
    pub exclude_elements: Vec<String>,
}

impl Default for HtmlSegmentConfig {
    fn default() -> Self {
        Self {
            language_code: "en".to_string(),
            add_marks: true,
            exclude_elements: vec![
                "script".to_string(),
                "style".to_string(),
                "noscript".to_string(),
                "iframe".to_string(),
                "svg".to_string(),
                "math".to_string(),
                "pre".to_string(),
                "code".to_string(),
                "textarea".to_string(),
                "title".to_string(),
                "meta".to_string(),
                "link".to_string(),
                "base".to_string(),
                "head".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone)]
struct TextNodeInfo {
    /// Unique identifier for this node
    node_id: usize,
    /// Start byte position in the original HTML
    start_byte: usize,
    /// End byte position in the original HTML (for debugging/future use)
    #[allow(dead_code)]
    end_byte: usize,
    /// The raw text content of this node
    raw_text: String,
}

/// Mapping between flattened text positions and original node positions
#[derive(Debug, Clone)]
struct NodeLinearRange {
    /// Node identifier
    node_id: usize,
    /// Start position in the flattened text
    global_start: usize,
    /// End position in the flattened text (exclusive)
    global_end: usize,
}

/// Represents a sentence slice within a specific text node
#[derive(Debug, Clone)]
struct PerNodeSentenceSlice {
    /// Sentence identifier
    sentence_id: usize,
    /// Node identifier
    node_id: usize,
    /// Start position within the node's text
    local_start: usize,
    /// End position within the node's text (exclusive)
    local_end: usize,
    /// Whether this slice covers the entire node
    covers_entire_node: bool,
}

/// A patch to apply to the original HTML
#[derive(Debug, Clone)]
enum Patch {
    /// Replace a byte range with new HTML content
    ReplaceRange {
        start_byte: usize,
        end_byte: usize,
        new_html: String,
    },
}

/// Result of HTML sentence segmentation
#[derive(Debug, Clone, Serialize)]
pub struct HtmlSentenceAnnotation {
    /// Sentence identifier
    pub id: usize,
    /// The sentence text (extracted from HTML)
    pub text: String,
    /// HTML ranges that make up this sentence
    pub html_ranges: Vec<HtmlRangePart>,
}

/// Part of an HTML range for a sentence
#[derive(Debug, Clone, Serialize)]
pub struct HtmlRangePart {
    /// Node identifier in the parsed tree
    pub node_id: usize,
    /// Start byte position in original HTML
    pub start_byte: usize,
    /// End byte position in original HTML
    pub end_byte: usize,
    /// Start position within the node's text
    pub local_start: usize,
    /// End position within the node's text
    pub local_end: usize,
}

/// Result of HTML sentence segmentation with optional marked HTML
#[derive(Debug, Clone, Serialize)]
pub struct HtmlSegmentationResult {
    /// List of sentence annotations
    pub sentences: Vec<HtmlSentenceAnnotation>,
    /// HTML with `<mark>` tags added (if requested)
    pub marked_html: Option<String>,
}

/// Segment sentences in HTML content
///
/// This function parses HTML, extracts segmentable text, runs sentence segmentation,
/// and optionally adds `<mark>` tags around sentence boundaries.
///
/// # Arguments
///
/// * `html` - The HTML content to segment
/// * `config` - Configuration for segmentation
///
/// # Returns
///
/// A `HtmlSegmentationResult` containing sentence annotations and optionally marked HTML
///
/// # Example
///
/// ```
/// use sentencex::html::{segment_html, HtmlSegmentConfig};
///
/// let html = "<p>Hello world. This is a <em>test</em>.</p>";
/// let config = HtmlSegmentConfig::default();
/// let result = segment_html(html, &config).unwrap();
///
/// assert_eq!(result.sentences.len(), 2);
/// if let Some(marked) = result.marked_html {
///     println!("Marked HTML: {}", marked);
/// }
/// ```
pub fn segment_html(
    html: &str,
    config: &HtmlSegmentConfig,
) -> Result<HtmlSegmentationResult, Box<dyn std::error::Error>> {
    // Parse HTML with tree-sitter
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_html::LANGUAGE.into())?;
    let tree = parser.parse(html, None).ok_or("Failed to parse HTML")?;

    // Collect segmentable text nodes
    let text_nodes =
        collect_text_nodes(tree.root_node(), html.as_bytes(), &config.exclude_elements);

    // Build flattened text and mapping
    let (flattened_text, node_ranges) = build_flattened_text(&text_nodes);

    // Segment sentences on the flattened text
    let sentence_boundaries = get_sentence_boundaries(&config.language_code, &flattened_text);

    // Map sentence boundaries back to nodes
    let sentence_slices =
        map_boundaries_to_nodes(&sentence_boundaries, &node_ranges, &flattened_text);

    // Create sentence annotations
    let sentences = create_sentence_annotations(&sentence_slices, &text_nodes);

    // Generate marked HTML if requested
    let marked_html = if config.add_marks {
        Some(generate_marked_html(html, &sentence_slices, &text_nodes)?)
    } else {
        None
    };

    Ok(HtmlSegmentationResult {
        sentences,
        marked_html,
    })
}

/// Collect text nodes from the HTML tree, excluding specified elements
fn collect_text_nodes(node: Node, source: &[u8], exclude_elements: &[String]) -> Vec<TextNodeInfo> {
    let mut text_nodes = Vec::new();
    let mut node_counter = 0;

    // Pre-scan: collect excluded element byte ranges
    let mut excluded_ranges: Vec<(usize, usize)> = Vec::new();

    fn collect_excluded(
        node: Node,
        source: &[u8],
        exclude_elements: &[String],
        ranges: &mut Vec<(usize, usize)>,
    ) {
        match node.kind() {
            "script_element" | "style_element" => {
                ranges.push((node.start_byte(), node.end_byte()));
                return; // Skip entire subtree
            }
            "element" => {
                // Use our parse_element function to properly extract tag name
                if let Some(html_tag) = parse_element(&node, source) {
                    if exclude_elements.contains(&html_tag.name) {
                        ranges.push((node.start_byte(), node.end_byte()));
                        return; // Skip entire subtree
                    }
                }
            }
            _ => {}
        }

        // Recursively traverse children
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                collect_excluded(child, source, exclude_elements, ranges);
            }
        }
    }

    collect_excluded(node, source, exclude_elements, &mut excluded_ranges);

    // Merge overlapping excluded ranges for faster checks
    if !excluded_ranges.is_empty() {
        excluded_ranges.sort_by_key(|r| r.0);
        let mut merged: Vec<(usize, usize)> = Vec::new();
        let mut current = excluded_ranges[0];
        for &(s, e) in &excluded_ranges[1..] {
            if s <= current.1 {
                // overlap
                current.1 = current.1.max(e);
            } else {
                merged.push(current);
                current = (s, e);
            }
        }
        merged.push(current);
        excluded_ranges = merged;
    }

    fn is_excluded(start: usize, end: usize, ranges: &[(usize, usize)]) -> bool {
        // Simple linear search for overlap
        for &(rs, re) in ranges {
            if start < re && end > rs {
                return true;
            }
        }
        false
    }

    fn traverse(
        node: Node,
        source: &[u8],
        text_nodes: &mut Vec<TextNodeInfo>,
        node_counter: &mut usize,
        excluded_ranges: &[(usize, usize)],
    ) {
        if node.kind() == "text" {
            let start_byte = node.start_byte();
            let end_byte = node.end_byte();

            // Debug output for text nodes
            if let Ok(text) = node.utf8_text(source) {
                eprintln!(
                    "Text node: '{}' at bytes ({}, {})",
                    text, start_byte, end_byte
                );
            }

            if is_excluded(start_byte, end_byte, excluded_ranges) {
                eprintln!("  -> EXCLUDED");
                return; // Skip text inside excluded range
            }

            if let Ok(text) = node.utf8_text(source) {
                let trimmed = text.trim();
                eprintln!("  -> trimmed: '{}'", trimmed);
                if !trimmed.is_empty() {
                    text_nodes.push(TextNodeInfo {
                        node_id: *node_counter,
                        start_byte,
                        end_byte,
                        raw_text: text.to_string(),
                    });
                    *node_counter += 1;
                    eprintln!("  -> ADDED as node {}", *node_counter - 1);
                } else {
                    eprintln!("  -> SKIPPED (empty after trim)");
                }
            }
        }

        // Descend only if node is not fully excluded
        let nstart = node.start_byte();
        let nend = node.end_byte();
        if is_excluded(nstart, nend, excluded_ranges) && node.kind() == "element" {
            return;
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                traverse(child, source, text_nodes, node_counter, excluded_ranges);
            }
        }
    }

    traverse(
        node,
        source,
        &mut text_nodes,
        &mut node_counter,
        &excluded_ranges,
    );
    text_nodes
}

/// Check if a text node has an excluded element as an ancestor (fallback / debug)
fn has_excluded_ancestor(node: Node, source: &[u8], exclude_elements: &[String]) -> bool {
    let mut current = node.parent();
    while let Some(parent) = current {
        if parent.kind() == "element" {
            if let Some(tag_node) = parent.child_by_field_name("start_tag") {
                if let Some(name_node) = tag_node.child_by_field_name("name") {
                    let tag_name = name_node.utf8_text(source).unwrap_or("").to_lowercase();
                    if exclude_elements.contains(&tag_name) {
                        return true;
                    }
                }
            }
        }
        current = parent.parent();
    }
    false
}

/// Build flattened text from text nodes and create mapping
fn build_flattened_text(text_nodes: &[TextNodeInfo]) -> (String, Vec<NodeLinearRange>) {
    let mut flattened = String::new();
    let mut node_ranges = Vec::new();

    for node in text_nodes {
        let start = flattened.len();
        flattened.push_str(&node.raw_text);
        let end = flattened.len();

        node_ranges.push(NodeLinearRange {
            node_id: node.node_id,
            global_start: start,
            global_end: end,
        });
    }

    (flattened, node_ranges)
}

/// Map sentence boundaries back to individual nodes
fn map_boundaries_to_nodes(
    boundaries: &[SentenceBoundary],
    node_ranges: &[NodeLinearRange],
    _flattened_text: &str,
) -> Vec<PerNodeSentenceSlice> {
    let mut slices = Vec::new();

    for (sentence_id, boundary) in boundaries.iter().enumerate() {
        let sent_start = boundary.start_index;
        let sent_end = boundary.end_index;

        // Find all nodes that overlap with this sentence
        for range in node_ranges {
            if range.global_end <= sent_start || range.global_start >= sent_end {
                continue; // No overlap
            }

            // Calculate the intersection
            let local_start = if sent_start <= range.global_start {
                0
            } else {
                sent_start - range.global_start
            };

            let local_end = if sent_end >= range.global_end {
                range.global_end - range.global_start
            } else {
                sent_end - range.global_start
            };

            let covers_entire_node =
                local_start == 0 && local_end == (range.global_end - range.global_start);

            slices.push(PerNodeSentenceSlice {
                sentence_id,
                node_id: range.node_id,
                local_start,
                local_end,
                covers_entire_node,
            });
        }
    }

    slices
}

/// Create sentence annotations from the slices
fn create_sentence_annotations(
    slices: &[PerNodeSentenceSlice],
    text_nodes: &[TextNodeInfo],
) -> Vec<HtmlSentenceAnnotation> {
    let mut sentence_map: HashMap<usize, Vec<&PerNodeSentenceSlice>> = HashMap::new();

    // Group slices by sentence ID
    for slice in slices {
        sentence_map
            .entry(slice.sentence_id)
            .or_default()
            .push(slice);
    }

    let mut sentences = Vec::new();

    for (sentence_id, sentence_slices) in sentence_map {
        let mut text = String::new();
        let mut html_ranges = Vec::new();

        for slice in sentence_slices {
            if let Some(node) = text_nodes.iter().find(|n| n.node_id == slice.node_id) {
                let slice_text = &node.raw_text[slice.local_start..slice.local_end];
                text.push_str(slice_text);

                html_ranges.push(HtmlRangePart {
                    node_id: slice.node_id,
                    start_byte: node.start_byte + slice.local_start,
                    end_byte: node.start_byte + slice.local_end,
                    local_start: slice.local_start,
                    local_end: slice.local_end,
                });
            }
        }

        sentences.push(HtmlSentenceAnnotation {
            id: sentence_id,
            text,
            html_ranges,
        });
    }

    // Sort by sentence ID to maintain order
    sentences.sort_by_key(|s| s.id);
    sentences
}

/// Generate HTML with `<mark>` tags using Strategy A (node-local marking) with merging
fn generate_marked_html(
    original_html: &str,
    slices: &[PerNodeSentenceSlice],
    text_nodes: &[TextNodeInfo],
) -> Result<String, Box<dyn std::error::Error>> {
    let mut patches = Vec::new();

    // Strategy A: Create patches for each slice
    for slice in slices {
        if let Some(node) = text_nodes.iter().find(|n| n.node_id == slice.node_id) {
            let abs_start = node.start_byte + slice.local_start;
            let abs_end = node.start_byte + slice.local_end;
            let original = &original_html[abs_start..abs_end];
            let marked_text = format!(
                "<mark data-sent=\"{}\"{}>{}</mark>",
                slice.sentence_id,
                if slice.covers_entire_node {
                    " data-full-node=\"true\""
                } else {
                    ""
                },
                original
            );

            patches.push(Patch::ReplaceRange {
                start_byte: abs_start,
                end_byte: abs_end,
                new_html: marked_text,
            });
        }
    }

    // Sort patches by start position (reverse order for easier application)
    patches.sort_by_key(|p| match p {
        Patch::ReplaceRange { start_byte, .. } => std::cmp::Reverse(*start_byte),
    });

    // Apply patches to generate marked HTML
    let mut result = original_html.to_string();

    for patch in patches {
        match patch {
            Patch::ReplaceRange {
                start_byte,
                end_byte,
                new_html,
            } => {
                // Ensure we don't go out of bounds
                if start_byte <= result.len() && end_byte <= result.len() && start_byte <= end_byte
                {
                    result.replace_range(start_byte..end_byte, &new_html);
                }
            }
        }
    }

    // Simple safe merging pass
    result = merge_consecutive_marks_safely(&result);

    Ok(result)
}

/// Merge consecutive `<mark>` tags with the same `data-sent` attribute
fn merge_consecutive_marks_safely(html: &str) -> String {
    let mut s = html.to_string();
    let mut i = 0;
    let close_pat = "</mark><mark data-sent=\"";
    while let Some(idx) = s[i..].find(close_pat) {
        let idx = i + idx;
        // parse next id N
        let id_start = idx + close_pat.len();
        if let Some(id_end_quote) = s[id_start..].find('"') {
            let id_end = id_start + id_end_quote;
            let id_n = &s[id_start..id_end];
            // find end of the opening tag '>'
            if let Some(gt_off) = s[id_end..].find('>') {
                let open_end = id_end + gt_off + 1; // position after '>'
                // find the previous opening <mark ...> before idx
                if let Some(prev_open_idx) = s[..idx].rfind("<mark ") {
                    // extract previous id M
                    if let Some(attr_idx) = s[prev_open_idx..idx].find("data-sent=\"") {
                        let vstart = prev_open_idx + attr_idx + "data-sent=\"".len();
                        if let Some(vend_off) = s[vstart..idx].find('"') {
                            let vend = vstart + vend_off;
                            let id_m = &s[vstart..vend];
                            if id_m == id_n {
                                // remove boundary: </mark><mark data-sent="N"...>
                                s.replace_range(idx..open_end, "");
                                // continue scanning from same position
                                continue;
                            }
                        }
                    }
                }
                // advance past this boundary if not merged
                i = open_end;
                continue;
            } else {
                break; // malformed
            }
        } else {
            break; // malformed
        }
    }
    s
}

/// Escape HTML special characters
fn html_escape(text: &str) -> String {
    // Not used in current pipeline; kept for future use.
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_html_segmentation() {
        let html = "<p>Hello world. This is a test.</p>";
        let config = HtmlSegmentConfig {
            add_marks: false,
            ..Default::default()
        };

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);
        assert_eq!(result.sentences[0].text.trim(), "Hello world.");
        assert_eq!(result.sentences[1].text.trim(), "This is a test.");
        assert!(result.marked_html.is_none());
    }

    #[test]
    fn test_html_with_inline_elements() {
        let html = "<p>This is a <em>bold statement</em> that continues here. This is another sentence.</p>";
        let config = HtmlSegmentConfig {
            add_marks: false,
            ..Default::default()
        };

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);
        assert!(result.sentences[0].text.contains("bold statement"));
        assert!(result.sentences[1].text.contains("another sentence"));
    }

    #[test]
    fn test_excluded_elements() {
        let html = r#"
            <p>This is text.</p>
            <script>alert("This should not be segmented.");</script>
            <p>More text here.</p>
        "#;
        let config = HtmlSegmentConfig {
            add_marks: false,
            ..Default::default()
        };

        let result = segment_html(html, &config).unwrap();

        // Should only find sentences from <p> elements, not <script>
        assert_eq!(result.sentences.len(), 2);
        assert!(result.sentences[0].text.contains("This is text"));
        assert!(result.sentences[1].text.contains("More text here"));
    }

    #[test]
    fn test_marked_html_generation() {
        let html = "<p>Hello world. This is a test.</p>";
        let config = HtmlSegmentConfig {
            add_marks: true,
            ..Default::default()
        };

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);
        assert!(result.marked_html.is_some());

        let marked = result.marked_html.unwrap();
        assert!(marked.contains(r#"<mark data-sent="0""#));
        assert!(marked.contains(r#"<mark data-sent="1""#));
        assert!(marked.contains("Hello world."));
        assert!(marked.contains("This is a test."));
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("Hello & <world>"), "Hello &amp; &lt;world&gt;");
        assert_eq!(
            html_escape(r#"He said "Hello""#),
            "He said &quot;Hello&quot;"
        );
    }

    #[test]
    fn test_merge_consecutive_marks_safely() {
        let html = r#"<p><mark data-sent="0">Hello </mark><mark data-sent="0">world.</mark></p>"#;
        let merged = merge_consecutive_marks_safely(html);
        assert_eq!(merged, r#"<p><mark data-sent="0">Hello world.</mark></p>"#);

        // different ids should not merge
        let html2 = r#"<p><mark data-sent="0">Hello.</mark><mark data-sent="1"> World.</mark></p>"#;
        let merged2 = merge_consecutive_marks_safely(html2);
        assert_eq!(merged2, html2);
    }

    #[test]
    fn test_partial_content_inside_tags() {
        let html = "<p>The <strong>quick brown fox jumps. Over</strong> the lazy dog.</p>";
        let config = HtmlSegmentConfig {
            add_marks: false,
            ..Default::default()
        };

        let result = segment_html(html, &config).unwrap();

        assert_eq!(result.sentences.len(), 2);
        assert!(result.sentences[0].text.contains("quick brown fox jumps"));
        assert!(
            result.sentences[1].text.contains("Over")
                && result.sentences[1].text.contains("lazy dog")
        );
    }
}
