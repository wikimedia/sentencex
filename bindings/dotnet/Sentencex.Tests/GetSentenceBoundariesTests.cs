using System.Linq;
using Xunit;

namespace Sentencex.Tests;

public class GetSentenceBoundariesTests
{
    [Fact]
    public void BasicBoundaries()
    {
        string text = "Hello world. This is a test.";
        SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries("en", text);

        Assert.Equal(2, boundaries.Length);

        Assert.Equal("Hello world. ", boundaries[0].Text);
        Assert.Equal(0, boundaries[0].StartIndex);
        Assert.Equal(13, boundaries[0].EndIndex);

        Assert.Equal("This is a test.", boundaries[1].Text);
        Assert.Equal(13, boundaries[1].StartIndex);
        Assert.Equal(28, boundaries[1].EndIndex);
    }

    [Fact]
    public void BoundaryProperties()
    {
        string text = "Hello! World?";
        SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries("en", text);

        foreach (SentenceBoundary boundary in boundaries)
        {
            Assert.NotNull(boundary.Text);
            Assert.True(boundary.StartIndex >= 0);
            Assert.True(boundary.EndIndex > boundary.StartIndex);
            Assert.IsType<bool>(boundary.IsParagraphBreak);
            Assert.NotNull(boundary.BoundarySymbol);
        }

        Assert.Equal("!", boundaries[0].BoundarySymbol);
        Assert.Equal("?", boundaries[1].BoundarySymbol);
    }

    [Fact]
    public void ParagraphBreaks()
    {
        string text = "First paragraph.\n\nSecond paragraph.";
        SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries("en", text);

        Assert.True(boundaries[1].IsParagraphBreak);
    }

    [Fact]
    public void EmptyText()
    {
        SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries("en", "");
        Assert.Empty(boundaries);
    }

    [Fact]
    public void BoundaryIndicesConsistency()
    {
        string text = "First sentence. Second sentence. Third sentence.";
        SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries("en", text);

        string reconstructed = string.Concat(boundaries.Select(b => b.Text));
        Assert.Equal(text, reconstructed);

        for (int i = 1; i < boundaries.Length; i++)
            Assert.Equal(boundaries[i - 1].EndIndex, boundaries[i].StartIndex);
    }

    [Fact]
    public void BoundarySymbolIsSetForTerminatedSentences()
    {
        string text = "Hello world. This is a test.";
        SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries("en", text);

        Assert.All(boundaries, b => Assert.NotNull(b.BoundarySymbol));
    }

    [Fact]
    public void TextWithExclamationAndQuestion()
    {
        string text = "What is this? It's a test! Amazing.";
        SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries("en", text);

        Assert.Equal(3, boundaries.Length);
        Assert.Equal(0, boundaries[0].StartIndex);
        Assert.Equal(boundaries[0].EndIndex, boundaries[1].StartIndex);
        Assert.Equal(boundaries[1].EndIndex, boundaries[2].StartIndex);
        Assert.Equal(text.Length, boundaries[2].EndIndex);
    }

    [Fact]
    public void GetSentenceBoundariesInvalidLanguageFallback()
    {
        string text = "Hello world. This is a test.";
        SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries("invalid_lang", text);
        SentenceBoundary[] boundariesEn = Segmenter.GetSentenceBoundaries("en", text);

        Assert.Equal(boundariesEn.Length, boundaries.Length);
        Assert.Equal(boundariesEn[0].StartIndex, boundaries[0].StartIndex);
        Assert.Equal(boundariesEn[1].EndIndex, boundaries[1].EndIndex);
    }
}
