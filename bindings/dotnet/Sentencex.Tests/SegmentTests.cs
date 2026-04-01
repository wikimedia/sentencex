using System.Linq;
using Xunit;

namespace Sentencex.Tests;

public class SegmentTests
{
    [Fact]
    public void BasicSegmentation()
    {
        string text = "Hello world. This is a test.";
        string[] sentences = Segmenter.Segment("en", text);

        Assert.Equal(2, sentences.Length);
        Assert.Equal("Hello world. ", sentences[0]);
        Assert.Equal("This is a test.", sentences[1]);
    }

    [Fact]
    public void EmptyText()
    {
        string[] sentences = Segmenter.Segment("en", "");
        Assert.Empty(sentences);
    }

    [Fact]
    public void SingleSentence()
    {
        string text = "This is a single sentence";
        string[] sentences = Segmenter.Segment("en", text);

        Assert.Single(sentences);
        Assert.Equal("This is a single sentence", sentences[0]);
    }

    [Fact]
    public void MultiplePunctuation()
    {
        string text = "What is this? It's a test! Amazing.";
        string[] sentences = Segmenter.Segment("en", text);

        Assert.Equal(3, sentences.Length);
        Assert.Equal("What is this? ", sentences[0]);
        Assert.Equal("It's a test! ", sentences[1]);
        Assert.Equal("Amazing.", sentences[2]);
    }

    [Fact]
    public void Abbreviations()
    {
        string text = "Dr. Smith went to the U.S. yesterday. He had a great time.";
        string[] sentences = Segmenter.Segment("en", text);

        Assert.Equal(2, sentences.Length);
        Assert.Contains("Dr. Smith went to the U.S. yesterday. ", sentences[0]);
        Assert.Contains("He had a great time.", sentences[1]);
    }

    [Theory]
    [InlineData("en", "Hello world. This is English.", 2)]
    [InlineData("es", "Hola mundo. Esto es español.", 2)]
    [InlineData("fr", "Bonjour le monde. C'est du français.", 2)]
    public void DifferentLanguages(string language, string text, int expectedCount)
    {
        string[] sentences = Segmenter.Segment(language, text);
        Assert.Equal(expectedCount, sentences.Length);
    }

    [Fact]
    public void ReconstructedTextMatchesOriginal()
    {
        string text = "First sentence. Second sentence. Third sentence.";
        string[] sentences = Segmenter.Segment("en", text);

        string reconstructed = string.Concat(sentences);
        Assert.Equal(text, reconstructed);
    }

    [Fact]
    public void VeryLongText()
    {
        string longText = string.Concat(Enumerable.Range(0, 100).Select(i => $"This is sentence {i}. "));
        string[] sentences = Segmenter.Segment("en", longText);

        Assert.Equal(100, sentences.Length);
    }

    [Fact]
    public void InvalidLanguageCodeFallback()
    {
        string text = "Hello world. This is a test.";
        string[] sentences = Segmenter.Segment("invalid_lang", text);
        string[] sentencesEn = Segmenter.Segment("en", text);

        Assert.Equal(2, sentences.Length);
        Assert.Equal(sentencesEn.Length, sentences.Length);
        Assert.Equal(sentencesEn[0], sentences[0]);
        Assert.Equal(sentencesEn[1], sentences[1]);
    }

    [Fact]
    public void UnicodeText()
    {
        string text = "Hello 世界. This is a test with émojis 🌍.";
        string[] sentences = Segmenter.Segment("en", text);

        Assert.Equal(2, sentences.Length);
        Assert.Equal("Hello 世界. ", sentences[0]);
        Assert.Equal("This is a test with émojis 🌍.", sentences[1]);
    }

    [Fact]
    public void TextWithVariousNewlineCharacters()
    {
        string text = "First line.\nSecond line.\r\nThird line.\r\rFourth line.";
        string[] sentences = Segmenter.Segment("en", text);

        Assert.Equal(4, sentences.Length);
    }
}
