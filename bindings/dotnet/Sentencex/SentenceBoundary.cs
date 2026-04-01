namespace Sentencex;

/// <summary>
/// Represents a sentence boundary detected within an input text.
/// </summary>
public sealed class SentenceBoundary
{
    /// <summary>Character index where the sentence starts.</summary>
    public int StartIndex { get; }

    /// <summary>Character index where the sentence ends.</summary>
    public int EndIndex { get; }

    /// <summary>The sentence text.</summary>
    public string Text { get; }

    /// <summary>The punctuation mark that ended the sentence, or <see langword="null"/> if none.</summary>
    public string? BoundarySymbol { get; }

    /// <summary><see langword="true"/> if this boundary represents a paragraph break.</summary>
    public bool IsParagraphBreak { get; }

    internal SentenceBoundary(int startIndex, int endIndex, string text, string? boundarySymbol, bool isParagraphBreak)
    {
        StartIndex = startIndex;
        EndIndex = endIndex;
        Text = text;
        BoundarySymbol = boundarySymbol;
        IsParagraphBreak = isParagraphBreak;
    }
}