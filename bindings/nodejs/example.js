import { segment, get_sentence_boundaries } from "sentencex";

const language_code = "en";
const text = "Hello world. This is a test.";
const sentences = segment(language_code, text);
console.log("Segmented sentences:", sentences);

const boundaries = get_sentence_boundaries(
  "en",
  "This is first sentence. This is another one.",
);
console.log(boundaries);

import { readFileSync } from "fs";

const shakespeare = readFileSync(
  "../../benchmarks/fixtures/shakespeare.txt",
  "utf-8",
);
const t = performance.now();
const shakespeare_sentences = segment(language_code, shakespeare);
const timeTaken = performance.now() - t;
console.log(`Speed : ${timeTaken.toFixed(2).padStart(20)} ms`);
console.log(`Sentences: ${shakespeare_sentences.length}`);
