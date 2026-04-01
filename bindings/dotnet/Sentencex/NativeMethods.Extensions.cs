using System;
using System.Text;

namespace Sentencex.Native;

internal unsafe partial struct SegmentResult
{
    /// <summary>Returns a <see cref="ReadOnlySpan{T}"/> over the native <see cref="ByteRange"/> entries.</summary>
    public ReadOnlySpan<ByteRange> AsReadOnlySpan() => new ReadOnlySpan<ByteRange>(ptr, (int)len);
}

internal unsafe partial struct BoundaryResult
{
    /// <summary>Returns a <see cref="ReadOnlySpan{T}"/> over the native <see cref="BoundaryEntry"/> entries.</summary>
    public ReadOnlySpan<BoundaryEntry> AsReadOnlySpan() => new ReadOnlySpan<BoundaryEntry>(ptr, (int)len);
}

internal unsafe partial struct BoundaryEntry
{
    /// <summary>
    /// Decodes the boundary symbol as a UTF-8 string.
    /// <see langword="null"/> when there is no boundary symbol (<see cref="boundary_symbol_len"/> is 0).
    /// </summary>
    public string? BoundarySymbol
    {
        get
        {
            if (boundary_symbol_len == 0)
                return null;

            fixed (byte* p = boundary_symbol)
                return Encoding.UTF8.GetString(p, boundary_symbol_len);
        }
    }
}

internal static unsafe partial class NativeMethods
{
    public static unsafe SegmentResult InvokeSegment(byte[] languageBytes, int languageByteCount, byte[] textBytes, int textByteCount)
    {
        fixed (byte* languagePtr = languageBytes)
        {
            fixed (byte* textPtr = textBytes)
            {
                return NativeMethods.sentencex_segment(languagePtr, (nuint)languageByteCount, textPtr, (nuint)textByteCount);
            }
        }
    }

    public static unsafe BoundaryResult InvokeGetBoundaries(byte[] languageBytes, int languageByteCount, byte[] textBytes, int textByteCount)
    {
        fixed (byte* languagePtr = languageBytes)
        {
            fixed (byte* textPtr = textBytes)
            {
                return NativeMethods.sentencex_get_boundaries(languagePtr, (nuint)languageByteCount, textPtr, (nuint)textByteCount);
            }
        }
    }
}
