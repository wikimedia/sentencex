# Sentence segmenter

[![tests](https://github.com/santhoshtr/sentencesegmenter/actions/workflows/tests.yaml/badge.svg)](https://github.com/santhoshtr/sentencesegmenter/actions/workflows/tests.yaml)

Basic approach:

- If it's a period, it ends a sentence.
- If the preceding token is in the hand-compiled list of abbreviations, then it doesn't end a sentence.
- If the next token is capitalized, then it ends a sentence.

This is library is based on the following principle:

> When in doubt, do not split.

If two sentences are accidentally together, that is ok. It is better than
sentence being split in middle. Avoid over engineering to get everything
linguistically 100% accurate.

This approach would be suitable for applications like text to speech, machine translation.

Consider this example: `We make a good team, you and I. Did you see Albert I. Jones yesterday?`

The accurate splitting of this sentence is
`["We make a good team, you and I." ,"Did you see Albert I. Jones yesterday?"]`

However, to achieve this level precision, complex rules need to be added and it could create side effects. Instead, if we just don't segment between `I. Did`, it is ok for most of downstream applications.

## Language support

The aim is to support all languages where there is a wikipedia. Instead of falling back on English for languages not defined in the library, a fallback chain is used. The closest language which is defined in the library will be used.

## Performance

Measured on Golden Rule Set for English. Lists are excempted(1. sentence 2. another sentence).

| Tokenizer                |  GRS score    | Speed(Avg over 100 runs) |
|--------------------------|------------|-----------|
| sentencesegmenter_segment |    74.36  |     0.93 |
| mwtokenizer_tokenize      |    30.77  |    1.54  |
| blingfire_tokenize        |    89.74  |    **0.27**  |
| nltk_tokenize             |    66.67  |    1.86  |
| pysbd_tokenize            |**97.44**  |    10.57 |
| spacy_tokenize            |    61.54  |     2.45 |
| spacy_dep_tokenize        |   74.36   |   138.93 |
| stanza_tokenize           |   87.18   |   107.51 |
| syntok_tokenize           |    79.49  |     4.72 |

## License

MIT license. See [License.txt](./LICENSE.txt)
