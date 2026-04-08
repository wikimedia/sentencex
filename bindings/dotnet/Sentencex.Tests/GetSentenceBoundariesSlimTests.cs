using System.Linq;
using Xunit;

namespace Sentencex.Tests;

public class GetSentenceBoundariesSlimTests
{
    [Fact]
    public void BasicBoundaries()
    {
        string text = "Hello world. This is a test.";
        SentenceBoundarySlim[] boundaries = Segmenter.GetSentenceBoundariesSlim("en", text);

        Assert.Equal(2, boundaries.Length);

        Assert.Equal(0, boundaries[0].StartIndex);
        Assert.Equal(13, boundaries[0].EndIndex);

        Assert.Equal(13, boundaries[1].StartIndex);
        Assert.Equal(28, boundaries[1].EndIndex);
    }

    [Fact]
    public void EmptyText()
    {
        SentenceBoundarySlim[] boundaries = Segmenter.GetSentenceBoundariesSlim("en", "");
        Assert.Empty(boundaries);
    }

    [Fact]
    public void BoundaryIndicesConsistency()
    {
        string text = "First sentence. Second sentence. Third sentence.";
        SentenceBoundarySlim[] boundaries = Segmenter.GetSentenceBoundariesSlim("en", text);

        string reconstructed = string.Concat(boundaries.Select(b => text[b.StartIndex..b.EndIndex]));
        Assert.Equal(text, reconstructed);

        for (int i = 1; i < boundaries.Length; i++)
            Assert.Equal(boundaries[i - 1].EndIndex, boundaries[i].StartIndex);
    }

    [Fact]
    public void IndicesMatchFullBoundaries()
    {
        string text = "Hello world. This is a test.";
        SentenceBoundary[] full = Segmenter.GetSentenceBoundaries("en", text);
        SentenceBoundarySlim[] slim = Segmenter.GetSentenceBoundariesSlim("en", text);

        Assert.Equal(full.Length, slim.Length);

        for (int i = 0; i < full.Length; i++)
        {
            Assert.Equal(full[i].StartIndex, slim[i].StartIndex);
            Assert.Equal(full[i].EndIndex, slim[i].EndIndex);
        }
    }

    [Fact]
    public void UnicodeTextIndices()
    {
        string text = "Hello 世界. This is a test with émojis 🌍.";
        SentenceBoundarySlim[] boundaries = Segmenter.GetSentenceBoundariesSlim("en", text);

        Assert.Equal(2, boundaries.Length);
        Assert.Equal(0, boundaries[0].StartIndex);
        Assert.Equal(text.Length, boundaries[^1].EndIndex);

        string reconstructed = string.Concat(boundaries.Select(b => text[b.StartIndex..b.EndIndex]));
        Assert.Equal(text, reconstructed);
    }

    [Fact]
    public void GetSentenceBoundariesSlimInvalidLanguageFallback()
    {
        string text = "Hello world. This is a test.";
        SentenceBoundarySlim[] boundaries = Segmenter.GetSentenceBoundariesSlim("invalid_lang", text);
        SentenceBoundarySlim[] boundariesEn = Segmenter.GetSentenceBoundariesSlim("en", text);

        Assert.Equal(boundariesEn.Length, boundaries.Length);
        Assert.Equal(boundariesEn[0].StartIndex, boundaries[0].StartIndex);
        Assert.Equal(boundariesEn[1].EndIndex, boundaries[1].EndIndex);
    }
}
