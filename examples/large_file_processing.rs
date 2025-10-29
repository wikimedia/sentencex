use sentencex::{segment, segment_borrowed, segment_chunked};
use std::fs::File;
use std::time::Instant;

fn main() {
    // Create a sample large text file
    println!("Creating test files...");
    let small_text = generate_test_text(1_000); // 1KB
    let medium_text = generate_test_text(100_000); // ~100KB
    let large_text = generate_test_text(1_000_000); // ~1MB

    // Write test files
    std::fs::write("/tmp/small.txt", &small_text).unwrap();
    std::fs::write("/tmp/medium.txt", &medium_text).unwrap();
    std::fs::write("/tmp/large.txt", &large_text).unwrap();

    println!("\n=== Performance Comparison ===\n");

    // Test 1: Small file (1KB)
    println!("Small file (1KB):");
    test_segment_performance("en", &small_text);

    // Test 2: Medium file (100KB)
    println!("\nMedium file (100KB):");
    test_segment_performance("en", &medium_text);

    // Test 3: Large file (1MB) - only with borrowed variant
    println!("\nLarge file (1MB):");
    println!("  segment_borrowed: {}", time_operation(|| {
        segment_borrowed("en", &large_text)
    }));

    // Test 4: Using chunked API for very large files
    println!("\n=== Chunked Processing (for very large files) ===");
    let file = File::open("/tmp/large.txt").unwrap();
    let start = Instant::now();
    let sentences = segment_chunked("en", file, 65536).unwrap();
    let duration = start.elapsed();
    println!("  Processed 1MB file in {:?}", duration);
    println!("  Found {} sentences", sentences.len());

    // Demonstrate memory efficiency
    println!("\n=== Memory Efficiency Demo ===");
    demonstrate_memory_efficiency();
}

fn generate_test_text(size: usize) -> String {
    let base = "The James Webb Space Telescope is revolutionary. \
                Dr. Smith works at NASA. \
                The meeting is at 3 p.m. on Jan. 15th. \
                She said, \"This is amazing!\" \n\n";
    
    let mut text = String::with_capacity(size);
    while text.len() < size {
        text.push_str(base);
    }
    text.truncate(size);
    text
}

fn test_segment_performance(lang: &str, text: &str) {
    println!("  segment: {}", time_operation(|| {
        segment(lang, text)
    }));
    
    println!("  segment_borrowed: {}", time_operation(|| {
        segment_borrowed(lang, text)
    }));
}

fn time_operation<F, T>(f: F) -> String 
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let _ = f();
    let duration = start.elapsed();
    format!("{:?}", duration)
}

fn demonstrate_memory_efficiency() {
    let text = "First sentence. Second sentence. Third sentence.";
    
    // Using segment() - allocates new strings
    let owned_sentences = segment("en", text);
    println!("  segment() returns Vec<String>:");
    println!("    - Allocates new strings for each sentence");
    println!("    - Example: {:?}", owned_sentences);
    
    // Using segment_borrowed() - zero-copy, just references
    let borrowed_sentences = segment_borrowed("en", text);
    println!("\n  segment_borrowed() returns Vec<&str>:");
    println!("    - Zero-copy: returns references to original text");
    println!("    - More efficient for large texts");
    println!("    - Example: {:?}", borrowed_sentences);
    
    // Verify they produce the same content
    assert_eq!(owned_sentences.len(), borrowed_sentences.len());
    for (owned, borrowed) in owned_sentences.iter().zip(borrowed_sentences.iter()) {
        assert_eq!(owned.as_str(), *borrowed);
    }
    println!("\n  ✓ Both methods produce identical results");
}
