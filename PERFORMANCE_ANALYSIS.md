# Performance Analysis of SentenceX Rust Library

## Executive Summary

This document presents a comprehensive performance analysis of the SentenceX sentence segmentation library, including identified bottlenecks and proposed optimizations, particularly for large text processing.

## Current Performance Baseline

### Small Text Performance (< 1KB)
- **100 bytes**: ~14.15 µs (6.74 MiB/s throughput)
- **1,000 bytes**: ~108.76 µs (8.77 MiB/s throughput)

### Medium Text Performance (10KB)
- **10,000 bytes**: ~5.71 ms (1.67 MiB/s throughput)

### Large Text Performance (100KB+)
- **100,000 bytes**: ~532 ms (183 KiB/s throughput) ⚠️ **CRITICAL PERFORMANCE ISSUE**
- **1,000,000 bytes**: Estimated >50 seconds ⚠️ **UNACCEPTABLE FOR PRODUCTION**

## Performance Degradation Analysis

The performance degradation is **non-linear** and worsens significantly with text size:
- 10x increase (100B → 1KB): ~7.7x slower
- 10x increase (1KB → 10KB): ~52x slower  
- 10x increase (10KB → 100KB): ~93x slower

This indicates **O(n²) or worse complexity** rather than the expected O(n).

## Identified Bottlenecks

### 1. **String Allocation in `segment()` Method**
**Location**: `src/languages/language.rs:170-180`

**Issue**: The `segment()` method calls `get_sentence_boundaries()` and then allocates new `String` objects for each sentence:
```rust
pub fn segment(&self, text: &str) -> Vec<String> {
    let boundaries = self.get_sentence_boundaries(text);
    for boundary in boundaries {
        sentences.push(boundary.text.to_string());  // ← Allocation for each sentence
    }
}
```

**Impact**: For large texts with thousands of sentences, this creates thousands of heap allocations.

### 2. **Regex Compilation and Matching**
**Location**: `src/languages/language.rs:81-86`

**Issue**: Multiple regex operations on the same text:
- `get_sentence_break_regex().find_iter()`: Scans entire paragraph
- `QUOTES_REGEX.find_iter()`: Scans entire paragraph  
- `PARENS_REGEX.find_iter()`: Scans entire paragraph
- `EMAIL_REGEX.find_iter()`: Scans entire paragraph

Each regex scan is O(n), and with 4 different regex patterns, we're effectively doing 4 full scans.

### 3. **Quadratic Complexity in Range Checking**
**Location**: `src/languages/language.rs:98-108`

**Issue**: For each potential sentence boundary found, we check against all skippable ranges:
```rust
for (start, end) in matches {
    // ...
    for (range_start, range_end) in &skippable_ranges {  // ← Nested loop
        if boundary > *range_start && boundary < *range_end {
            // ...
        }
    }
}
```

This is O(m × r) where m = number of matches and r = number of ranges.

### 4. **Character Boundary Calculations**
**Location**: `src/languages/language.rs:132-155`

**Issue**: Finding the last character for boundary symbol detection walks backwards character by character:
```rust
let mut char_end = end;
while char_end > 0 && !paragraph.is_char_boundary(char_end) {
    char_end -= 1;  // ← Potentially many iterations for multi-byte chars
}
```

For UTF-8 text with multi-byte characters, this can be inefficient.

### 5. **Vector Reallocations**
**Issue**: While capacity pre-allocation has been added, it may not be sufficient for very large texts.

## Proposed Optimizations

### High Priority (Expected 5-10x improvement for large files)

#### 1. **Implement Streaming/Chunking for Large Files**
- Process text in chunks (e.g., 64KB blocks) to maintain constant memory usage
- Use a sliding window approach to handle sentence boundaries that span chunks
- **Expected Impact**: Enable processing of multi-GB files with constant memory

#### 2. **Optimize Regex Operations**
- Combine multiple regex patterns into a single pass where possible
- Use `regex::RegexSet` for parallel matching
- Consider lazy evaluation of skippable ranges
- **Expected Impact**: 2-3x improvement on regex-heavy operations

#### 3. **Use Lazy Caching for Language Objects**
- Cache compiled `Language` objects to avoid repeated regex compilation
- Use `Arc<Language>` for thread-safe sharing
- **Expected Impact**: Significant improvement for repeated calls with same language

### Medium Priority (Expected 2-3x improvement)

#### 4. **Optimize Skippable Range Checking**
- Sort ranges and use binary search instead of linear search
- Use interval trees for O(log n) range queries
- **Expected Impact**: Improves from O(n×m) to O(n×log m)

#### 5. **Zero-Copy String Slicing**
- Return `&str` slices instead of `String` where possible
- Add a zero-copy variant: `segment_borrowed()` returning `Vec<&str>`
- **Expected Impact**: Eliminates thousands of allocations for large texts

#### 6. **Optimize Character Boundary Detection**
- Cache character positions during iteration
- Use `str::char_indices()` instead of manual boundary checking
- **Expected Impact**: Small improvement, better code clarity

### Low Priority (Polish and edge cases)

#### 7. **Parallel Processing**
- Use `rayon` for parallel paragraph processing
- Split text into paragraphs and process in parallel
- **Expected Impact**: Near-linear speedup with CPU cores (2-4x on typical systems)

#### 8. **SIMD Optimizations**
- Use SIMD for character classification (whitespace, punctuation)
- Requires nightly Rust or external crates like `packed_simd`
- **Expected Impact**: Marginal improvement, high complexity

## Recommendations for Large File Processing

### Immediate Actions
1. **Implement chunk-based processing** - This is critical for production use
2. **Add streaming API** - `segment_stream()` that yields sentences incrementally
3. **Document memory usage** - Add guidance on expected memory consumption

### API Additions
```rust
// Streaming API for large files
pub fn segment_stream<'a>(language_code: &str, text: &'a str) -> impl Iterator<Item = &'a str>;

// Zero-copy variant for better performance
pub fn segment_borrowed<'a>(language_code: &str, text: &'a str) -> Vec<&'a str>;

// Chunk-based processing
pub fn segment_chunked(language_code: &str, reader: impl Read) -> Result<Vec<String>>;
```

### Configuration Options
```rust
pub struct SegmentConfig {
    pub chunk_size: usize,        // For large file processing
    pub max_sentence_length: usize, // Safety limit
    pub parallel: bool,            // Enable parallel processing
}
```

## Testing Recommendations

1. **Add benchmarks for real-world scenarios**:
   - Wikipedia articles (various sizes)
   - Books (Project Gutenberg)
   - News articles
   - Social media posts

2. **Add memory profiling**:
   - Track allocations using `dhat` or `valgrind`
   - Measure peak memory usage

3. **Add stress tests**:
   - Very large files (>100MB)
   - Pathological cases (many abbreviations, quotes)
   - Unicode-heavy texts

## Conclusion

The current implementation works well for small to medium texts (< 10KB) but has **critical performance issues** with large texts due to:
1. Non-linear complexity (O(n²) behavior)
2. Excessive string allocations
3. Multiple full-text regex scans

The proposed optimizations, particularly streaming/chunking and regex optimization, should improve large file performance by **50-100x**, making it suitable for production use with large documents.

## Next Steps

1. Implement high-priority optimizations
2. Create comprehensive benchmark suite (completed)
3. Profile with real-world data
4. Document performance characteristics
5. Add streaming API for large file support
