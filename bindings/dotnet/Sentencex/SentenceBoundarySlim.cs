namespace Sentencex;

/// <summary>
/// Lightweight representation of a sentence boundary containing only character index information.
/// </summary>
public readonly struct SentenceBoundarySlim
{
    /// <summary>Character index where the sentence starts.</summary>
    public int StartIndex { get; }

    /// <summary>Character index where the sentence ends.</summary>
    public int EndIndex { get; }

    internal SentenceBoundarySlim(int startIndex, int endIndex)
    {
        StartIndex = startIndex;
        EndIndex = endIndex;
    }
}
