# sentencex

Python bindings for [SentenceX](https://github.com/wikimedia/sentencex), a high-performance multilingual sentence segmentation library written in Rust.

## Installation

```bash
pip install sentencex
```

## Usage

```python
import sentencex

sentences = sentencex.segment("en", "This is first sentence. This is another one.")
print(sentences)
# ['This is first sentence. ', 'This is another one.']
```

### Get Sentence Boundaries

For detailed boundary information:

```python
import sentencex

boundaries = sentencex.get_sentence_boundaries("en", "This is first sentence. This is another one.")
for boundary in boundaries:
    print(boundary)
# {'start_index': 0, 'end_index': 24, 'text': 'This is first sentence. ', ...}
```

## API

- `segment(language_code: str, text: str) -> List[str]` — Segment text into sentences
- `get_sentence_boundaries(language_code: str, text: str) -> List[Dict]` — Get detailed boundary information

## Language Support

Supports ~244 languages with automatic fallback chains. See the [upstream documentation](https://github.com/wikimedia/sentencex#language-support) for details.

## Performance

See [upstream benchmarks](https://github.com/wikimedia/sentencex#performance) for comparison with other libraries.

## License

MIT. See [https://github.com/wikimedia/sentencex](https://github.com/wikimedia/sentencex)
