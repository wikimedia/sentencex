using Sentencex.Native;
using System;
using System.Buffers;
using System.Text;

namespace Sentencex;

/// <summary>
/// Sentence segmentation utilities powered by the sentencex native library.
/// </summary>
public static class Segmenter
{
    /// <summary>
    /// Segments <paramref name="text"/> into sentences for the given <paramref name="language"/>.
    /// </summary>
    /// <param name="language">BCP 47 language code (e.g. "en", "fr", "ja").</param>
    /// <param name="text">The text to segment.</param>
    /// <returns>An array of sentence strings.</returns>
    public static string[] Segment(string language, string text)
    {
        int languageByteCount = Encoding.UTF8.GetByteCount(language);
        byte[] languageBytes = ArrayPool<byte>.Shared.Rent(languageByteCount);

        int textByteCount = Encoding.UTF8.GetByteCount(text);
        byte[] textBytes = ArrayPool<byte>.Shared.Rent(textByteCount);

        try
        {
            Encoding.UTF8.GetBytes(language.AsSpan(), languageBytes);
            Encoding.UTF8.GetBytes(text.AsSpan(), textBytes);

            SegmentResult result = NativeMethods.InvokeSegment(languageBytes, languageByteCount, textBytes, textByteCount);

            try
            {
                ReadOnlySpan<ByteRange> entries = result.AsReadOnlySpan();

                if (entries.IsEmpty)
                    return Array.Empty<string>();

                var segments = new string[entries.Length];

                for (int i = 0; i < entries.Length; i++)
                {
                    var start = (int)entries[i].start;
                    var length = (int)(entries[i].end - entries[i].start);
                    segments[i] = Encoding.UTF8.GetString(textBytes, start, length);
                }

                return segments;
            }
            finally
            {
                NativeMethods.sentencex_free_segment_result(result);
            }
        }
        finally
        {
            ArrayPool<byte>.Shared.Return(languageBytes);
            ArrayPool<byte>.Shared.Return(textBytes);
        }
    }

    /// <summary>
    /// Returns detailed sentence boundaries for <paramref name="text"/> in the given <paramref name="language"/>.
    /// </summary>
    /// <param name="language">BCP 47 language code (e.g. "en", "fr", "ja").</param>
    /// <param name="text">The text to analyze.</param>
    /// <returns>An array of <see cref="SentenceBoundary"/> objects.</returns>
    public static SentenceBoundary[] GetSentenceBoundaries(string language, string text)
    {
        int langByteCount = Encoding.UTF8.GetByteCount(language);
        byte[] langBytes = ArrayPool<byte>.Shared.Rent(langByteCount);

        int textByteCount = Encoding.UTF8.GetByteCount(text);
        byte[] textBytes = ArrayPool<byte>.Shared.Rent(textByteCount);
        try
        {
            Encoding.UTF8.GetBytes(language.AsSpan(), langBytes);
            Encoding.UTF8.GetBytes(text.AsSpan(), textBytes);

            var result = NativeMethods.InvokeGetBoundaries(langBytes, langByteCount, textBytes, textByteCount);

            try
            {
                ReadOnlySpan<BoundaryEntry> entries = result.AsReadOnlySpan();

                if (entries.IsEmpty)
                    return Array.Empty<SentenceBoundary>();

                var boundaries = new SentenceBoundary[entries.Length];

                int bytePos = 0;
                int charPos = 0;

                for (int i = 0; i < entries.Length; i++)
                {
                    BoundaryEntry entry = entries[i];

                    int startByte = (int)entry.start_byte;
                    int endByte = (int)entry.end_byte;

                    charPos += Encoding.UTF8.GetCharCount(textBytes.AsSpan(bytePos, startByte - bytePos));
                    int startIndex = charPos;

                    string sentenceText = Encoding.UTF8.GetString(textBytes, startByte, endByte - startByte);
                    int endIndex = startIndex + sentenceText.Length;

                    charPos = endIndex;
                    bytePos = endByte;

                    boundaries[i] = new(
                        startIndex: startIndex,
                        endIndex: endIndex,
                        text: sentenceText,
                        boundarySymbol: entry.BoundarySymbol,
                        isParagraphBreak: entry.is_paragraph_break != 0);
                }

                return boundaries;
            }
            finally
            {
                NativeMethods.sentencex_free_boundary_result(result);
            }
        }
        finally
        {
            ArrayPool<byte>.Shared.Return(langBytes);
            ArrayPool<byte>.Shared.Return(textBytes);
        }
    }

    /// <summary>
    /// Returns the start and end character indices of each sentence in <paramref name="text"/> for the given <paramref name="language"/>.
    /// </summary>
    /// <param name="language">BCP 47 language code (e.g. "en", "fr", "ja").</param>
    /// <param name="text">The text to analyze.</param>
    /// <returns>An array of <see cref="SentenceBoundarySlim"/> values containing only index information.</returns>
    public static SentenceBoundarySlim[] GetSentenceBoundariesSlim(string language, string text)
    {
        int languageByteCount = Encoding.UTF8.GetByteCount(language);
        byte[] languageBytes = ArrayPool<byte>.Shared.Rent(languageByteCount);

        int textByteCount = Encoding.UTF8.GetByteCount(text);
        byte[] textBytes = ArrayPool<byte>.Shared.Rent(textByteCount);

        try
        {
            Encoding.UTF8.GetBytes(language.AsSpan(), languageBytes);
            Encoding.UTF8.GetBytes(text.AsSpan(), textBytes);

            BoundaryResult result = NativeMethods.InvokeGetBoundaries(languageBytes, languageByteCount, textBytes, textByteCount);

            try
            {
                ReadOnlySpan<BoundaryEntry> entries = result.AsReadOnlySpan();

                if (entries.IsEmpty)
                    return Array.Empty<SentenceBoundarySlim>();

                var boundaries = new SentenceBoundarySlim[entries.Length];

                int bytePos = 0;
                int charPos = 0;

                for (int i = 0; i < entries.Length; i++)
                {
                    int startByte = (int)entries[i].start_byte;
                    int endByte = (int)entries[i].end_byte;

                    charPos += Encoding.UTF8.GetCharCount(textBytes.AsSpan(bytePos, startByte - bytePos));
                    int startIndex = charPos;

                    charPos += Encoding.UTF8.GetCharCount(textBytes.AsSpan(startByte, endByte - startByte));
                    int endIndex = charPos;

                    bytePos = endByte;

                    boundaries[i] = new(startIndex, endIndex);
                }

                return boundaries;
            }
            finally
            {
                NativeMethods.sentencex_free_boundary_result(result);
            }
        }
        finally
        {
            ArrayPool<byte>.Shared.Return(languageBytes);
            ArrayPool<byte>.Shared.Return(textBytes);
        }
    }
}
