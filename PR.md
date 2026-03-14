# PR: Add support for »...« quotation marks

## Summary

This PR adds support for »...« (right-pointing guillemets) quotation marks used in German.

## Problem

Previously, the sentence segmentation failed to correctly handle text using »...« style quotes. For example:

```
»Er liest am liebsten historische Thriller«, antwortete sie schließlich. »Haben Sie etwas über das alte Rom?«
```

The text between » and « (including any sentence terminators) was not recognized as a quoted region, causing incorrect segmentation.

## Solution

Add the »→« quote pair to `get_quote_pairs()` in `src/constants.rs`.

## Changes

### src/constants.rs
- Add `quote_pairs.insert("»", "«");` to recognize right-pointing guillemets

### tests/de.txt
Add two test cases:
1. Mixed narration and quotes: Verifies proper segmentation when »...« quotes contain sentences
2. Consecutive quotes: Verifies proper segmentation of dialogue-style »...« »...« »...«

## Example Output

Before (incorrect):
With only `«`→`»` defined, the text `»...«` was mishandled. The first `«` would be incorrectly treated as a left quote, matching the next `»` and causing mis-segmentation of German quotes.

After (correct):
```
Input:
»Er liest am liebsten historische Thriller«, antwortete sie schließlich. »Haben Sie etwas über das alte Rom?«

Output:
»Er liest am liebsten historische Thriller«, antwortete sie schließlich.
»Haben Sie etwas über das alte Rom?«
```

## Languages Affected

This fix benefits:
- German (uses »...« in some typographic styles, alongside „...“)
