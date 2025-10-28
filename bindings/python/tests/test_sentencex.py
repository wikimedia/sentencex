"""Test cases for sentencex Python binding."""

import pytest
from sentencex import segment, get_sentence_boundaries


class TestSegment:
    """Test cases for the segment function."""

    def test_basic_segmentation(self):
        """Test basic sentence segmentation."""
        text = "Hello world. This is a test."
        sentences = segment("en", text)

        assert len(sentences) == 2
        assert sentences[0] == "Hello world. "
        assert sentences[1] == "This is a test."

    def test_empty_text(self):
        """Test segmentation with empty text."""
        sentences = segment("en", "")
        assert sentences == []

    def test_single_sentence(self):
        """Test segmentation with single sentence."""
        text = "This is a single sentence"
        sentences = segment("en", text)

        assert len(sentences) == 1
        assert sentences[0] == "This is a single sentence"

    def test_multiple_punctuation(self):
        """Test segmentation with multiple punctuation marks."""
        text = "What is this? It's a test! Amazing."
        sentences = segment("en", text)

        assert len(sentences) == 3
        assert sentences[0] == "What is this? "
        assert sentences[1] == "It's a test! "
        assert sentences[2] == "Amazing."

    def test_abbreviations(self):
        """Test that abbreviations don't break sentences incorrectly."""
        text = "Dr. Smith went to the U.S. yesterday. He had a great time."
        sentences = segment("en", text)

        # Should not split on "Dr." or "U.S."
        assert len(sentences) == 2
        assert "Dr. Smith went to the U.S. yesterday. " in sentences[0]
        assert "He had a great time." in sentences[1]

    def test_different_languages(self):
        """Test segmentation with different language codes."""
        # English
        en_text = "Hello world. This is English."
        en_sentences = segment("en", en_text)
        assert len(en_sentences) == 2

        # Spanish
        es_text = "Hola mundo. Esto es español."
        es_sentences = segment("es", es_text)
        assert len(es_sentences) == 2

        # French
        fr_text = "Bonjour le monde. C'est du français."
        fr_sentences = segment("fr", fr_text)
        assert len(fr_sentences) == 2


class TestGetSentenceBoundaries:
    """Test cases for the get_sentence_boundaries function."""

    def test_basic_boundaries(self):
        """Test basic sentence boundary detection."""
        text = "Hello world. This is a test."
        boundaries = get_sentence_boundaries("en", text)

        assert len(boundaries) == 2

        # First sentence
        assert boundaries[0]["text"] == "Hello world. "
        assert boundaries[0]["start_index"] == 0
        assert boundaries[0]["end_index"] == 13

        # Second sentence
        assert boundaries[1]["text"] == "This is a test."
        assert boundaries[1]["start_index"] == 13
        assert boundaries[1]["end_index"] == 28

    def test_boundary_properties(self):
        """Test that boundary objects have expected properties."""
        text = "Hello! World?"
        boundaries = get_sentence_boundaries("en", text)

        for boundary in boundaries:
            assert "text" in boundary
            assert "start_index" in boundary
            assert "end_index" in boundary
            assert "boundary_symbol" in boundary
            assert "is_paragraph_break" in boundary

            # Check types
            assert isinstance(boundary["text"], str)
            assert isinstance(boundary["start_index"], int)
            assert isinstance(boundary["end_index"], int)
            assert isinstance(boundary["is_paragraph_break"], bool)

    def test_paragraph_breaks(self):
        """Test paragraph break detection."""
        text = "First paragraph.\n\nSecond paragraph."
        boundaries = get_sentence_boundaries("en", text)

        # Should detect paragraph break
        paragraph_breaks = [b for b in boundaries if b["is_paragraph_break"]]
        assert len(paragraph_breaks) > 0

    def test_empty_text_boundaries(self):
        """Test boundary detection with empty text."""
        boundaries = get_sentence_boundaries("en", "")
        assert boundaries == []

    def test_boundary_indices_consistency(self):
        """Test that boundary indices are consistent."""
        text = "First sentence. Second sentence. Third sentence."
        boundaries = get_sentence_boundaries("en", text)

        # Reconstruct text from boundaries
        reconstructed = "".join([b["text"] for b in boundaries])
        assert reconstructed == text

        # Check that indices are sequential
        for i in range(1, len(boundaries)):
            assert boundaries[i - 1]["end_index"] == boundaries[i]["start_index"]


class TestErrorCases:
    """Test error handling and edge cases."""

    def test_invalid_language_code(self):
        """Test with invalid language code - should not crash."""
        text = "Hello world. This is a test."
        # Should not raise an exception, should fall back gracefully
        sentences = segment("invalid_lang", text)
        assert isinstance(sentences, list)

    def test_unicode_text(self):
        """Test with Unicode text."""
        text = "Hello 世界. This is a test with émojis 🌍."
        sentences = segment("en", text)

        assert len(sentences) == 2
        assert "世界" in sentences[0]
        assert "🌍" in sentences[1]

    def test_very_long_text(self):
        """Test with relatively long text."""
        long_text = "This is sentence {}. " * 100
        long_text = long_text.format(*range(100))

        sentences = segment("en", long_text)
        assert len(sentences) == 100

    def test_text_with_newlines(self):
        """Test text with various newline characters."""
        text = "First line.\nSecond line.\r\nThird line.\r\rFourth line."
        sentences = segment("en", text)

        # Should handle different newline types
        assert len(sentences) >= 4


if __name__ == "__main__":
    pytest.main([__file__])
