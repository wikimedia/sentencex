# Sentence segmenter

[![crates.io](https://img.shields.io/crates/v/sentencex)](https://crates.io/crates/sentencex)
[![PyPI](https://img.shields.io/pypi/v/sentencex)](https://pypi.org/project/sentencex/)
[![npm](https://img.shields.io/npm/v/sentencex)](https://www.npmjs.com/package/sentencex)
[![Rust Tests](https://github.com/wikimedia/sentencex/actions/workflows/rust.yml/badge.svg)](https://github.com/wikimedia/sentencex/actions/workflows/rust.yml)
[![Node.js Tests](https://github.com/wikimedia/sentencex/actions/workflows/node.yml/badge.svg)](https://github.com/wikimedia/sentencex/actions/workflows/node.yml)
[![Python Tests](https://github.com/wikimedia/sentencex/actions/workflows/python.yml/badge.svg)](https://github.com/wikimedia/sentencex/actions/workflows/python.yml)
[![C# Tests](https://github.com/wikimedia/sentencex/actions/workflows/dotnet.yml/badge.svg)](https://github.com/wikimedia/sentencex/actions/workflows/dotnet.yml)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/wikimedia/sentencex)

A sentence segmentation library written in Rust language with wide language support optimized for speed and utility.

## Bindings

Besides native Rust, bindings for the following programming languages are available:

* [Python](https://pypi.org/project/sentencex/)
* [Nodejs](https://www.npmjs.com/package/sentencex)
* [Web(Wasm)](https://www.npmjs.com/package/sentencex-wasm)
* [C#](https://www.nuget.org/packages/sentencex/)

## Approach

- A sentence-terminating punctuation mark (`.`, `?`, `!`, plus language specific terminators like `।` or `။`) ends a sentence by default. Some languages use different terminators so a list of known terminator symbols is maintained for supported languages.
- Hand-compiled abbreviation lists, exclamation words, numbered references (`See [1]. Next sentence.`), and quote-aware rules (see below) suppress or relocate boundaries where the default rule would over-split. We collect a list of known, popular abbreviations in supported languages.
- List-item starts (e.g., bullets `*` / `+` / `-` / `•`, numeric `1.` / `1)` / `(1)`, lettered `a)` / `(a)`, roman `ii.`) emit sentence boundaries so each item segments cleanly, even when items are written inline on one line. A sibling rule (≥2 matches of the same marker family per paragraph, or a single Tier-1 line-start) keeps prose with stray `(1894)` or `e. e. cummings` from being mis-split.
- Multi-character punctuation runs (`. . .`, `! ?`, `? ? ?`, glued or space-separated) are treated as a single terminator. This generalises the ellipsis (`…` / `...`) case: any mix of `.`, `!`, `?` - repeated, spaced, or interleaved - collapses into one boundary candidate instead of several. Continuation heuristics then decide whether the following token starts a new sentence: uppercase non-`I` splits, while lowercase, digits, or glued continuations (e.g. `mean...see`) keep the sentence intact.
- Starter-word overrides recover boundaries that abbreviation and name-initial rules would otherwise suppress. When a suppressed terminator is followed by a known sentence-starter word, the break is reinstated. Languages opt in by overriding a trait method and shipping a starter-word list; English is currently the only language with one.

Sometimes, it is very hard to get the segmentation correct. In such cases this library is opinionated and prefer not segmenting than wrong segmentation. If two sentences are accidentally together, that is ok. It is better than sentence being split in middle.
Avoid over engineering to get everything linguistically 100% accurate.

This approach would be suitable for applications like text to speech, machine translation.

### Trade-offs

The opinionated *don't-over-split* stance coexists with several rules that *do* recover real boundaries (numbered references, quote-aware handling, abbreviation/exclamation suppression). The aim is to be conservative where context is ambiguous, while still picking up structural signals that make a split safe.

Consider this example: `We make a good team, you and I. Did you see Albert I. Jones yesterday?`

The accurate splitting is
`["We make a good team, you and I.", "Did you see Albert I. Jones yesterday?"]`

The hard part is that the same `I.` shape appears twice in the sentence and has to be read differently each time. The trailing `I.` of the first clause is a real sentence terminator; the `I.` in `Albert I. Jones` is a name initial and the boundary there must stay suppressed. Structurally the two tokens are indistinguishable - telling them apart reliably needs semantic understanding of the surrounding noun phrase, which is outside the scope of a rule-based segmenter and fits ML approaches better. So by default the name-initial detector takes the conservative line and suppresses both, leaving the two sentences joined. For most downstream applications that is fine.

The starter-word override is a narrow carve-out to that posture. When a suppressed terminator is followed by a token that appears in a small, curated list of known sentence-starters, the boundary is restored. `Did` is on the English starter list, so it splits at that point. `Jones` is not, hence `Albert I. Jones` stays joined. The match is case-sensitive on the following token. A lowercase variant like `…you and I. did you see…` is still left joined - a lowercase opener is a weaker signal and the conservative default applies. On the flip side, common words that are also abbreviations like `man`, `mass`, `wash`, are omitted from the abbreviation list as they collide too often. Starter words do not provide a strong enough signal in that situation, so the conservative posture applies in the other direction. Therefore an example like `Even the most brilliant strategy can be derailed by the unpredictable nature of man.  Mistakes happen.`
will still split as `["Even the most brilliant strategy can be derailed by the unpredictable nature of man.  ", "Mistakes happen."]`

List-item detection follows the same conservative posture. Ambiguous inline shapes that collide with prose (bare `1.` / `a.` / `ii.` closers inline, single-letter `a.` patterns that look like initials, parenthesised numbers like `(1894)` that read as years) deliberately do not trigger list segmentation. A sibling rule (≥2 matches of the same marker family per paragraph, or a single Tier-1 line-start) further suppresses one-off occurrences. The result: real lists segment per item, but prose containing list-shaped fragments stays intact.

Several other small heuristics follow the same "recover when the signal is clear, otherwise leave it joined" posture:

- **Stray punctuation around terminators**: a period immediately followed by a comma (`…ice cream. , It was…`) is treated as stray punctuation and the sentence continues through it. Whitespace between an abbreviation and its terminator (`U.S .`) is tolerated when looking up the abbreviation, so the boundary is still suppressed.
- **Dot-coded tokens like chess notation**: tokens of the shape `<digit>.<letter…>` (e.g. `7.Bg5`, `1.e4`) do not emit a boundary, so move codes and similar dot-coded identifiers stay inside their sentence.
- **Slash-joined abbreviations**: tokens like `171/U.S.` are split on `/` when extracting the trailing word, so the abbreviation on the right-hand side is still recognised.

The sentence segmentation in this library is **non-destructive**. This means, if the sentences are combined together, you can reconstruct the original text. Line breaks, punctuations and whitespaces are preserved in the output.

## Usage

### Rust

Install the library using

```bash
cargo add sentencex
```

Then, any text can be segmented as follows.

```rust
use sentencex::segment;

fn main() {
    let text = "The James Webb Space Telescope (JWST) is a space telescope specifically designed to conduct infrared astronomy. The U.S. National Aeronautics and Space Administration (NASA) led Webb's design and development.";
    let sentences = segment("en", text);

    for (i, sentence) in sentences.iter().enumerate() {
        println!("{}. {}", i + 1, sentence);
    }
}
```

The first argument is language code, second argument is text to segment. The `segment` method returns an array of identified sentences.

### Python

Install from PyPI:

```bash
pip install sentencex
```

```python
import sentencex

text = "The James Webb Space Telescope (JWST) is a space telescope specifically designed to conduct infrared astronomy. The U.S. National Aeronautics and Space Administration (NASA) led Webb's design and development."

# Segment text into sentences
sentences = sentencex.segment("en", text)
for i, sentence in enumerate(sentences, 1):
    print(f"{i}. {sentence}")

# Get sentence boundaries with indices
boundaries = sentencex.get_sentence_boundaries("en", text)
for boundary in boundaries:
    print(f"Sentence: '{boundary['text']}' (indices: {boundary['start_index']}-{boundary['end_index']})")
```

See [bindings/python/example.py](bindings/python/example.py) for more examples.

### Node.js

Install from npm:

```bash
npm install sentencex
```

```javascript
import { segment, get_sentence_boundaries } from 'sentencex';

const text = "The James Webb Space Telescope (JWST) is a space telescope specifically designed to conduct infrared astronomy. The U.S. National Aeronautics and Space Administration (NASA) led Webb's design and development.";

// Segment text into sentences
const sentences = segment("en", text);
sentences.forEach((sentence, i) => {
    console.log(`${i + 1}. ${sentence}`);
});

// Get sentence boundaries with indices
const boundaries = get_sentence_boundaries("en", text);
boundaries.forEach(boundary => {
    console.log(`Sentence: '${boundary.text}' (indices: ${boundary.start_index}-${boundary.end_index})`);
});
```

For CommonJS usage:

```javascript
const { segment, get_sentence_boundaries } = require('sentencex');
```

See [bindings/nodejs/example.js](bindings/nodejs/example.js) for more examples.

### WebAssembly (Browser)

Install from npm:

```bash
npm install sentencex-wasm
```

or use a CDN like `https://esm.sh/sentencex-wasm`

```javascript
import init, { segment, get_sentence_boundaries } from 'https://esm.sh/sentencex-wasm;

async function main() {
    // Initialize the WASM module
    await init();

    const text = "The James Webb Space Telescope (JWST) is a space telescope specifically designed to conduct infrared astronomy. The U.S. National Aeronautics and Space Administration (NASA) led Webb's design and development.";

    // Segment text into sentences
    const sentences = segment("en", text);
    sentences.forEach((sentence, i) => {
        console.log(`${i + 1}. ${sentence}`);
    });

    // Get sentence boundaries with indices
    const boundaries = get_sentence_boundaries("en", text);
    boundaries.forEach(boundary => {
        console.log(`Sentence: '${boundary.text}' (indices: ${boundary.start_index}-${boundary.end_index})`);
    });
}

main();
```

### C# / .NET 10+

Install from nuget:

```bash
dotnet add package sentencex
```

```csharp
using Sentencex;

string language = "en";
string inputText = "The James Webb Space Telescope (JWST) is a space telescope specifically designed to conduct infrared astronomy. The U.S. National Aeronautics and Space Administration (NASA) led Webb's design and development.";
string[] sentences = Segmenter.Segment("en", inputText);

// Segment text into sentences
foreach (string sentence in sentences)
    Console.WriteLine($"Sentence: {sentence}");

// Get sentence boundaries with indices and text
SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries(language, inputText);
foreach (SentenceBoundary boundary in boundaries)
    Console.WriteLine($"Sentence: '{boundary.Text}' (indices: {boundary.StartIndex}-{boundary.EndIndex})");

// Get sentence boundary indices without text
SentenceBoundarySlim[] boundariesSlim = Segmenter.GetSentenceBoundariesSlim(language, inputText);
foreach (SentenceBoundarySlim boundary in boundariesSlim)
    Console.WriteLine($"Sentence indices: {boundary.StartIndex}-{boundary.EndIndex}");
```

## Language support

The aim is to support all languages where there is a wikipedia. Instead of falling back on English for languages not defined in the library, a fallback chain is used. The closest language which is defined in the library will be used. Fallbacks for ~244 languages are defined.

## Performance

Following is a sample output of sentence segmenting [The Complete Works of William Shakespeare](https://www.gutenberg.org/files/100/100-0.txt).
This file is 5.29MB. As you can see below, it took 81 milli second.

```bash
$ curl https://www.gutenberg.org/files/100/100-0.txt | ./target/release/sentencex -l en > /dev/null
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 5295k  100 5295k    0     0   630k      0  0:00:08  0:00:08 --:--:-- 1061k
Time taken for segment(): 81.939969ms
Total sentences: 150671
```


Measured on English Golden Rule Set (GRS) using mean F1 score across 60 test cases.
The benchmark script is at [`benchmarks/compare.py`](benchmarks/compare.py) and can be run with `uv run benchmarks/compare.py`.

The following libraries are compared:

- [mwtokenizer](https://pypi.org/project/mwtokenizer/) — Wikimedia rule-based tokenizer
- [blingfire](https://github.com/microsoft/BlingFire) — Microsoft's fast tokenizer (C library)
- [nltk](https://pypi.org/project/nltk/) — Punkt sentence tokenizer
- [pysbd](https://github.com/nipunsadvilkar/pySBD/) — Python port of pragmatic segmenter
- [spacy](https://spacy.io/) — dependency-parse based sentence segmentation
- [syntok](https://github.com/fnl/syntok) — rule-based tokenizer

| Tokenizer   | English GRS F1 Score |
| ----------- | -------------------- |
| sentencex   | **100.00**           |
| pysbd       | 93.00                |
| blingfire   | 91.67                |
| syntok      | 85.67                |
| spacy       | 81.67                |
| mwtokenizer | 78.00                |
| nltk        | 72.33                |

## Thanks

- <https://github.com/diasks2/pragmatic_segmenter> for test cases. The English golden rule set is also sourced from it.
- <https://github.com/mush42/tqsm> for an earlier Rust port of this library.

## License

MIT license. See [License.txt](./LICENSE)
