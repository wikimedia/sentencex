# Sentence segmenter

[![crates.io](https://img.shields.io/crates/v/sentencex)](https://crates.io/crates/sentencex)
[![PyPI](https://img.shields.io/pypi/v/sentencex)](https://pypi.org/project/sentencex/)
[![npm](https://img.shields.io/npm/v/sentencex)](https://www.npmjs.com/package/sentencex)
[![Rust Tests](https://github.com/wikimedia/sentencex/actions/workflows/rust.yml/badge.svg)](https://github.com/wikimedia/sentencex/actions/workflows/rust.yml)
[![Node.js Tests](https://github.com/wikimedia/sentencex/actions/workflows/node.yaml/badge.svg)](https://github.com/wikimedia/sentencex/actions/workflows/node.yaml)
[![Python Tests](https://github.com/wikimedia/sentencex/actions/workflows/python.yaml/badge.svg)](https://github.com/wikimedia/sentencex/actions/workflows/python.yaml)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/wikimedia/sentencex)

A sentence segmentation library written in Rust language with wide language support optimized for speed and utility.

## Bindings

Besides native Rust, bindings for the following programming languages are available:

* [Python](https://pypi.org/project/sentencex/)
* [Nodejs](https://www.npmjs.com/package/sentencex)
* [Web(Wasm)](https://www.npmjs.com/package/sentencex-wasm)

## Approach

- If it's a period, it ends a sentence.
- If the preceding token is in the hand-compiled list of abbreviations, then it doesn't end a sentence.

However, it is not 'period' for many languages. So we will use a list of known punctuations that can cause a sentence break in as many languages as possible.

We also collect a list of known, popular abbreviations in as many languages as possible.

Sometimes, it is very hard to get the segmentation correct. In such cases this library is opinionated and prefer not segmenting than wrong segmentation. If two sentences are accidentally together, that is ok. It is better than sentence being split in middle.
Avoid over engineering to get everything linguistically 100% accurate.

This approach would be suitable for applications like text to speech, machine translation.

Consider this example: `We make a good team, you and I. Did you see Albert I. Jones yesterday?`

The accurate splitting of this sentence is
`["We make a good team, you and I." ,"Did you see Albert I. Jones yesterday?"]`

However, to achieve this level precision, complex rules need to be added and it could create side effects. Instead, if we just don't segment between `I. Did`, it is ok for most of downstream applications.

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


## Language support

The aim is to support all languages where there is a wikipedia. Instead of falling back on English for languages not defined in the library, a fallback chain is used. The closest language which is defined in the library will be used. Fallbacks for ~244 languages are defined.

## Performance

Following is a sample output of sentence segmenting [The Complete Works of William Shakespeare](https://www.gutenberg.org/files/100/100-0.txt).
This file is 5.29MB. As you can see below, it took half a second.

```bash
$ curl https://www.gutenberg.org/files/100/100-0.txt | ./target/release/sentencex -l en > /dev/null
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 5295k  100 5295k    0     0   630k      0  0:00:08  0:00:08 --:--:-- 1061k
Found 40923 paragraphs
Processing 540 chunks
Time taken for segment(): 521.071603ms
Total sentences: 153736
```


Measured on English Golden Rule Set (GRS) using mean F1 score across 60 test cases. List cases are excluded.
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
