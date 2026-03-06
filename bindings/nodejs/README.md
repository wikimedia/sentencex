# sentencex

Node.js bindings for [SentenceX](https://github.com/wikimedia/sentencex), a high-performance multilingual sentence segmentation library written in Rust.

## Installation

```bash
npm install sentencex
```

## Usage

```javascript
import { segment } from 'sentencex';

const sentences = segment("en", "This is first sentence. This is another one.");
console.log(sentences);
// [ 'This is first sentence. ', 'This is another one.' ]
```

### Get Sentence Boundaries

For detailed boundary information:

```javascript
import { get_sentence_boundaries } from 'sentencex';

const boundaries = get_sentence_boundaries("en", "This is first sentence. This is another one.");
console.log(boundaries);
// [ { start_index: 0, end_index: 24, text: 'This is first sentence. ' }, ... ]
```

### CommonJS

```javascript
const { segment } = require('sentencex');
```

## API

- `segment(languageCode: string, text: string): string[]` — Segment text into sentences
- `get_sentence_boundaries(languageCode: string, text: string): SentenceBoundary[]` — Get detailed boundary information

Each `SentenceBoundary` object contains:
- `start_index`: Character position where the sentence starts
- `end_index`: Character position where the sentence ends
- `text`: The sentence text
- `boundary_symbol`: Punctuation mark that ended the sentence (if any)
- `is_paragraph_break`: Whether this boundary represents a paragraph break

## Language Support

Supports ~244 languages with automatic fallback chains. See the [upstream documentation](https://github.com/wikimedia/sentencex#language-support) for details.

## Performance

See [upstream benchmarks](https://github.com/wikimedia/sentencex#performance) for comparison with other libraries.

## License

MIT. See [https://github.com/wikimedia/sentencex](https://github.com/wikimedia/sentencex)
