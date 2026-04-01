# Sentencex — .NET Bindings

[![NuGet](https://img.shields.io/nuget/v/sentencex)](https://www.nuget.org/packages/sentencex/)
[![C# Tests](https://github.com/wikimedia/sentencex/actions/workflows/dotnet.yml/badge.svg)](https://github.com/wikimedia/sentencex/actions/workflows/dotnet.yml)

.NET bindings for [sentencex](https://github.com/wikimedia/sentencex), a fast, multi-lingual sentence segmentation library written in Rust.

## Requirements

- .NET 10.0 or later

### Supported platforms
- Android 5.0+ (API level 21) for arm64, x64
- iOS 12.2+ for arm64
- Linux for x64, arm64 with glibc 2.35+ (e.g., Ubuntu 22.04+)
- Mac Catalyst 12.2+ for arm64, x64
- Mac OS 15.0+ for arm64, x64
- Windows 7+ for x64

## Installation

```bash
dotnet add package sentencex
```

## API

All methods live on the static `Segmenter` class in the `Sentencex` namespace.

### `Segmenter.Segment`

Splits text into an array of sentence strings.

```csharp
using Sentencex;

string text = "The James Webb Space Telescope (JWST) is a space telescope. The U.S. NASA led its development.";
string[] sentences = Segmenter.Segment("en", text);

foreach (string sentence in sentences)
    Console.WriteLine(sentence);
```

Output:
```
The James Webb Space Telescope (JWST) is a space telescope. 
The U.S. NASA led its development.
```

### `Segmenter.GetSentenceBoundaries`

```csharp
using Sentencex;

string text = "Hello world. This is a test.";
SentenceBoundary[] boundaries = Segmenter.GetSentenceBoundaries("en", text);

foreach (SentenceBoundary b in boundaries)
    Console.WriteLine($"[{b.StartIndex}–{b.EndIndex}] \"{b.Text}\" (boundary: '{b.BoundarySymbol}', paragraph: {b.IsParagraphBreak})");
```

Output

```
[0–13] "Hello world." (boundary: '.', paragraph: False)
[13–28] "This is a test." (boundary: '.', paragraph: False)
```

#### `SentenceBoundary` properties

| Property | Type | Description |
|---|---|---|
| `StartIndex` | `int` | Character index where the sentence starts |
| `EndIndex` | `int` | Character index where the sentence ends |
| `Text` | `string` | The sentence text |
| `BoundarySymbol` | `string?` | The punctuation mark that ended the sentence, or `null` if none |
| `IsParagraphBreak` | `bool` | `true` if this boundary represents a paragraph break |

### `Segmenter.GetSentenceBoundariesSlim`

Returns lightweight `SentenceBoundarySlim` values containing only start and end character indices. Use this when you only need index ranges and want to avoid allocating sentence text strings.

```csharp
using Sentencex;

string text = "Hello world. This is a test.";
SentenceBoundarySlim[] boundaries = Segmenter.GetSentenceBoundariesSlim("en", text);

foreach (SentenceBoundarySlim b in boundaries)
    Console.WriteLine($"[{b.StartIndex}–{b.EndIndex}] \"{text[b.StartIndex..b.EndIndex]}\"");
```

Output

```
[0–13] "Hello world."
[13–28] "This is a test."
```

#### `SentenceBoundarySlim` properties

| Property | Type | Description |
|---|---|---|
| `StartIndex` | `int` | Character index where the sentence starts |
| `EndIndex` | `int` | Character index where the sentence ends |

## Language support

The first argument to every method is a BCP 47 language code (e.g. `"en"`, `"fr"`, `"ja"`, `"ar"`).

Multiple languages are supported. See the [sentencex Rust library documentation](https://github.com/wikimedia/sentencex/blob/master/README.md#language-support) for more information.

## AOT compatibility

The library is AOT-compatible (`IsAotCompatible = true`).

## Building from source

The native library (`sentencex_dotnet`) must be compiled from the Rust source before running the managed project.

```bash
# From the bindings/dotnet directory
cargo build --release
dotnet build -c Release
dotnet test
```

## License
[MIT license](https://github.com/wikimedia/sentencex/blob/master/LICENSE)
