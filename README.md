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

Consider another example: `My id is Jane.`.
The correct splitting for this sentence is `["My id is Jane.", "Doe@example.com is my email" ]`. But getting it correct with a rule based system is hard. But there is no harm is not segmenting by strictly requiring a space after terminator for segmenting.

## License

MIT license. See [License.txt](./LICENSE.txt)
