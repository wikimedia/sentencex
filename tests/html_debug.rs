#[cfg(feature = "html")]
#[test]
fn debug_html_parsing() {
    use tree_sitter::Parser;
    use tree_sitter_html;

    let html = r#"<p>Normal text.</p><pre>Code here.</pre>"#;

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_html::LANGUAGE.into())
        .unwrap();
    let tree = parser.parse(html, None).unwrap();

    fn print_tree(node: tree_sitter::Node, source: &str, depth: usize) {
        let indent = "  ".repeat(depth);
        let kind = node.kind();
        let text = node.utf8_text(source.as_bytes()).unwrap_or("<invalid>");

        if kind == "text" {
            println!("{}text: '{}'", indent, text.trim());
        } else if kind == "element" {
            if let Some(tag_node) = node.child_by_field_name("start_tag") {
                if let Some(name_node) = tag_node.child_by_field_name("name") {
                    let tag_name = name_node.utf8_text(source.as_bytes()).unwrap_or("");
                    println!("{}{} ({})", indent, kind, tag_name);
                } else {
                    println!("{}{}", indent, kind);
                }
            } else {
                println!("{}{}", indent, kind);
            }
        } else {
            println!(
                "{}{}: {}",
                indent,
                kind,
                text.chars().take(30).collect::<String>()
            );
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                print_tree(child, source, depth + 1);
            }
        }
    }

    println!("HTML: {}", html);
    print_tree(tree.root_node(), html, 0);
}
