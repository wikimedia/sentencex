# Changelog

All notable changes to this project will be documented in this file.

## [1.0.14] - 2026-03-06

### Performance
- Fix O(n²) regex backtracking in QUOTES_REGEX by replacing `(\n|.)*?` with `(?s:.*?)`
- Fix per-paragraph regex allocation: move sentence-break regex to LazyLock static in default trait impl
- Fix Greek, Armenian, Burmese language modules which had no regex caching
- Improve large-text segmentation and benchmark stability
- Move paragraph-split regex to LazyLock static
- Extract continues_after_boundary helper to avoid per-call regex compilation

### Bug Fixes
- Fix boundary_symbol detection when followed by whitespace (#35)
- Fix get_next_word_approx called with full text instead of paragraph
- Fix CHUNK_SIZE in get_sentence_boundaries (1KB → 10KB)
- Fix boundary edge cases

### Testing
- Add comprehensive multi-byte character boundary tests

### Documentation
- Add badges to README
- Add detailed comments explaining char vs byte offset handling
- Simplify binding READMEs to be minimal and binding-specific

### Dependencies
- Remove unwanted dependencies, upgrade criterion

### Node.js
- Correct exports require mapping to use index.cjs

### CI
- Use macos-15-intel for x86 tests

### Misc
- Fix spelling: Telegu -> Telugu
- Minor formatting improvements

