import { test, describe } from "node:test";
import assert from "node:assert";
import { segment, get_sentence_boundaries } from "./index.mjs";

describe("sentencex", () => {
  describe("segment", () => {
    test("should segment simple English sentences", () => {
      const text = "Hello world. This is a test. How are you?";
      const result = segment("en", text);

      assert.strictEqual(Array.isArray(result), true);
      assert.strictEqual(result.length, 3);
      assert.strictEqual(result[0].trim(), "Hello world.");
      assert.strictEqual(result[1].trim(), "This is a test.");
      assert.strictEqual(result[2].trim(), "How are you?");
    });

    test("should handle single sentence", () => {
      const text = "This is one sentence.";
      const result = segment("en", text);

      assert.strictEqual(result.length, 1);
      assert.strictEqual(result[0].trim(), "This is one sentence.");
    });

    test("should handle empty string", () => {
      const text = "";
      const result = segment("en", text);

      assert.strictEqual(Array.isArray(result), true);
      assert.strictEqual(result.length, 0);
    });

    test("should handle text with abbreviations", () => {
      const text = "Dr. Smith went to the U.S.A. He had a great time.";
      const result = segment("en", text);

      assert.strictEqual(Array.isArray(result), true);
      assert(result.length >= 1);
    });

    test("should handle multiple languages", () => {
      const englishText = "Hello. World.";
      const englishResult = segment("en", englishText);

      const spanishText = "Hola. Mundo.";
      const spanishResult = segment("es", spanishText);

      assert.strictEqual(englishResult.length, 2);
      assert.strictEqual(spanishResult.length, 2);
    });
  });

  describe("get_sentence_boundaries", () => {
    test("should return sentence boundaries for English text", () => {
      const text = "Hello world. This is a test.";
      const result = get_sentence_boundaries("en", text);

      assert.strictEqual(Array.isArray(result), true);
      assert(result.length >= 2);

      // Check that each boundary has expected properties
      result.forEach((boundary) => {
        assert(typeof boundary === "object");
        assert(typeof boundary.start_index === "number");
        assert(typeof boundary.end_index === "number");
        assert(typeof boundary.text === "string");
      });
    });

    test("should handle empty string for boundaries", () => {
      const text = "";
      const result = get_sentence_boundaries("en", text);

      assert.strictEqual(Array.isArray(result), true);
      assert.strictEqual(result.length, 0);
    });

    test("should return correct indices for boundaries", () => {
      const text = "First. Second.";
      const result = get_sentence_boundaries("en", text);

      assert(result.length >= 2);

      // First boundary should start at 0
      assert.strictEqual(result[0].start_index, 0);

      // Boundaries should be in order
      for (let i = 1; i < result.length; i++) {
        assert(result[i].start_index >= result[i - 1].end_index);
      }
    });

    test("should extract correct text for boundaries", () => {
      const text = "Hello world. This is a test.";
      const result = get_sentence_boundaries("en", text);

      result.forEach((boundary) => {
        const extractedText = text.substring(
          boundary.start_index,
          boundary.end_index,
        );
        assert.strictEqual(boundary.text, extractedText);
      });
    });
  });

  describe("edge cases", () => {
    test("should handle text with only whitespace", () => {
      const text = "   \n\t  ";
      const segmentResult = segment("en", text);
      const boundariesResult = get_sentence_boundaries("en", text);

      assert.strictEqual(Array.isArray(segmentResult), true);
      assert.strictEqual(Array.isArray(boundariesResult), true);
    });

    test("should handle text with multiple consecutive punctuation", () => {
      const text = "What?! Really... Yes!!!";
      const result = segment("en", text);

      assert.strictEqual(Array.isArray(result), true);
      assert(result.length >= 1);
    });

    test("should handle newlines and paragraphs", () => {
      const text = "First paragraph.\n\nSecond paragraph.";
      const result = segment("en", text);
      const boundaries = get_sentence_boundaries("en", text);

      assert.strictEqual(Array.isArray(result), true);
      assert.strictEqual(Array.isArray(boundaries), true);
      assert(result.length >= 2);
    });
  });
});
