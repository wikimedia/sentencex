# Sentence segmenter

[![tests](https://github.com/wikimedia/sentencex/actions/workflows/rust.yml/badge.svg)](https://github.com/wikimedia/sentencex/actions/workflows/node.yml)

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

See [bindings/wasm/example.js](bindings/wasm/example.js) for more examples.

## Language support

The aim is to support all languages where there is a wikipedia. Instead of falling back on English for languages not defined in the library, a fallback chain is used. The closest language which is defined in the library will be used. Fallbacks for ~244 languages are defined.

## Performance

Measured on Golden Rule Set(GRS) for English. Lists are exempted (1. sentence 2. another sentence).

The following libraries are used for benchmarking:

- mwtokenizer from <https://gitlab.wikimedia.org/repos/research/wiki-nlp-tools>
- blingfire from <https://github.com/microsoft/BlingFire>
- nltk from <https://pypi.org/project/nltk/>
- pysbd from <https://github.com/nipunsadvilkar/pySBD/>
- spacy from <https://github.com/stanfordnlp/stanza>
- syntok from <https://github.com/fnl/syntok>

| Tokenizer Library    | English Golden Rule Set score | Speed(Avg over 100 runs) in seconds |
| -------------------- | ----------------------------- | ----------------------------------- |
| sentencex            | 74.36                         | **0.1357**                          |
| mwtokenizer_tokenize | 30.77                         | 1.54                                |
| blingfire_tokenize   | 89.74                         | 0.27                                |
| nltk_tokenize        | 66.67                         | 1.86                                |
| pysbd_tokenize       | **97.44**                     | 10.57                               |
| spacy_tokenize       | 61.54                         | 2.45                                |
| spacy_dep_tokenize   | 74.36                         | 138.93                              |
| stanza_tokenize      | 87.18                         | 107.51                              |
| syntok_tokenize      | 79.49                         | 4.72                                |

## Thanks

- <https://github.com/diasks2/pragmatic_segmenter> for test cases. The English golden rule set is also sourced from it.

## License

MIT license. See [License.txt](./LICENSE)
