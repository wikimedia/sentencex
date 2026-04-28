# Changelog

All notable changes to this project will be documented in this file.

## [1.0.23] - 2026-04-28

### CI
- Ignore temporary assets while publishing to crates.io

## [1.0.22] - 2026-04-27

### Added
- Add Ukrainian Support (#58)

### CI
- Add --allow-dirty to cargo publish in rust workflow

## [1.0.21] - 2026-04-24

### Added
- Add common Latin abbreviations and variations to English (e.g. ie, eg, abbrev.)

### Bug fixes
- Fix malformed test cases for Armenian (hy), Japanese (ja), and Russian (ru) (#57)

### CI
- Add Android wheel builds for Python (PEP 738 support)

## [1.0.20] - 2026-04-10

### Bug fixes
- Skip stale quote-extended boundaries (1435136)

## [1.0.19] - 2026-04-08

### Added
- Add .NET bindings with native library packaging and tests

## [1.0.18] - 2026-04-05

### Bug fixes
- Fix implicit chunk splitting breaking words and sentences when text exceeds 10KB with no paragraph breaks (#45)
- Add support for »...« quotation marks (a429c94)

## [1.0.17] - 2026-03-10

- Add Linux ARM64 build support (#41)

## [1.0.16] - 2026-03-06

### Documentation
- Style the demo page
- Update time taken for Shakespeare benchmark

### Maintenance
- Version bump

## [1.0.15] - 2026-03-06

### Node.js
- Do not run examples for test

### Documentation
- Simplify binding READMEs to be minimal and binding-specific

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
- Fix get_next_word_approx called with full text instead of paragraphUNK_SIZE in get
- Fix CH_sentence_boundaries (1KB → 10KB)
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
