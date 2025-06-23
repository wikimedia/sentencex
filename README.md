# Sentence segmenter

[![tests](https://github.com/wikimedia/sentencex-rust/actions/workflows/test.yml/badge.svg)](https://github.com/wikimedia/sentencex-rust/actions/workflows/test.yml)

A sentence segmentation library written in Rust language with wide language support optimized for speed and utility.

> This is a Rust language port of the original [sentencex](https://github.com/wikimedia/sentencex) library. A javascript/node port also exists [sentencex-js](https://github.com/wikimedia/sentencex-js)

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
