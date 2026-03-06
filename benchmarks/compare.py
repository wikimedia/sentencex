#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10,<3.14"
# dependencies = [
#   "sentencex",
#   "mwtokenizer",
#   "blingfire",
#   "numpy",
#   "nltk",
#   "pysbd",
#   "spacy",
#   "en-core-web-sm @ https://github.com/explosion/spacy-models/releases/download/en_core_web_sm-3.8.0/en_core_web_sm-3.8.0-py3-none-any.whl",
#   "syntok",
#   "stanza",
#   "torch",
# ]
#
# [[tool.uv.index]]
# name = "pytorch-cpu"
# url = "https://download.pytorch.org/whl/cpu"
# explicit = true
#
# [tool.uv.sources]
# torch = { index = "pytorch-cpu" }
# ///
"""
Sentence segmenter benchmark comparison.

Compares sentencex against other tokenizer libraries on:
  1. English Golden Rule Set (GRS) — F1 score, list cases excluded
  2. Gutenberg speed benchmark — The Complete Works of William Shakespeare (5.3MB)

Usage:
    uv run benchmarks/compare.py
    uv run benchmarks/compare.py --runs 3 --include-stanza
    uv run benchmarks/compare.py --grs-only
    uv run benchmarks/compare.py --speed-only
    uv run benchmarks/compare.py --output-format csv
"""

from __future__ import annotations

import argparse
import csv
import json
import os
import re
import sys
import time
import urllib.request
import warnings
from collections import Counter
from pathlib import Path
from typing import Callable

# ---------------------------------------------------------------------------
# Third-party imports — all done at module load so import time is excluded
# from benchmarking. Libraries that fail to import are stored as None and
# skipped at runtime.
# ---------------------------------------------------------------------------

with warnings.catch_warnings():
    warnings.simplefilter("ignore")

    try:
        import sentencex as _sentencex
    except ImportError:
        _sentencex = None  # type: ignore[assignment]

    try:
        from mwtokenizer.tokenizer import Tokenizer as _MWTokenizer
    except ImportError:
        _MWTokenizer = None  # type: ignore[assignment]

    try:
        import blingfire as _blingfire
    except ImportError:
        _blingfire = None  # type: ignore[assignment]

    try:
        import nltk as _nltk

        try:
            _nltk.data.find("tokenizers/punkt_tab")
        except LookupError:
            print("Downloading NLTK punkt_tab data...", flush=True)
            _nltk.download("punkt_tab", quiet=True)
    except ImportError:
        _nltk = None  # type: ignore[assignment]

    try:
        import pysbd as _pysbd
    except ImportError:
        _pysbd = None  # type: ignore[assignment]

    try:
        import spacy as _spacy

        _spacy_nlp = _spacy.load(
            "en_core_web_sm", disable=["ner", "lemmatizer", "attribute_ruler"]
        )
        _spacy_nlp_full = _spacy.load("en_core_web_sm")
    except (ImportError, OSError):
        _spacy = None  # type: ignore[assignment]
        _spacy_nlp = None
        _spacy_nlp_full = None

    try:
        import syntok.segmenter as _syntok_segmenter
    except ImportError:
        _syntok_segmenter = None  # type: ignore[assignment]

    try:
        import stanza as _stanza
    except ImportError:
        _stanza = None  # type: ignore[assignment]

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------

SCRIPT_DIR = Path(__file__).parent
REPO_ROOT = SCRIPT_DIR.parent
GRS_FILE = REPO_ROOT / "tests" / "en.txt"
FIXTURES_DIR = SCRIPT_DIR / "fixtures"
SHAKESPEARE_FILE = FIXTURES_DIR / "shakespeare.txt"
SHAKESPEARE_URL = "https://www.gutenberg.org/files/100/100-0.txt"

# ---------------------------------------------------------------------------
# GRS parsing
# ---------------------------------------------------------------------------


def _is_list_case(input_text: str) -> bool:
    """Return True if the test case input looks like a numbered/bulleted list."""
    list_patterns = [
        r"^\d+[\.\)]\)",  # 1.) or 1)
        r"^\d+\.",  # 1.
        r"^[•⁃]",  # bullet
        r"^[a-z]\.",  # a. b. c.
    ]
    for line in input_text.splitlines():
        line = line.strip()
        if line and any(re.match(p, line) for p in list_patterns):
            return True
    return False


def load_grs(path: Path) -> list[tuple[str, list[str]]]:
    """
    Parse tests/en.txt and return list of (input, expected_segments).
    Skips commented-out cases and list cases.
    """
    text = path.read_text(encoding="utf-8")
    cases = []
    for block in text.split("===\n"):
        block = block.strip()
        if not block or block.startswith("#"):
            continue
        if "---\n" not in block:
            continue
        input_part, expected_part = block.split("---\n", 1)
        input_text = input_part.strip()
        expected = [s.strip() for s in expected_part.strip().splitlines() if s.strip()]
        if not input_text or not expected:
            continue
        if _is_list_case(input_text):
            continue
        cases.append((input_text, expected))
    return cases


# ---------------------------------------------------------------------------
# F1 scoring
# ---------------------------------------------------------------------------


def f1_score(predicted: list[str], expected: list[str]) -> float:
    """
    Compute F1 between two lists of segments.
    Each segment is treated as a unit; uses multiset matching.
    Both sides are stripped before comparison.
    """
    pred = [s.strip() for s in predicted if s.strip()]
    exp = [s.strip() for s in expected if s.strip()]

    if not pred and not exp:
        return 1.0
    if not pred or not exp:
        return 0.0

    common = sum((Counter(pred) & Counter(exp)).values())
    precision = common / len(pred)
    recall = common / len(exp)
    if precision + recall == 0:
        return 0.0
    return 2 * precision * recall / (precision + recall)


def score_grs(
    tokenize_fn: Callable[[str], list[str]], cases: list[tuple[str, list[str]]]
) -> tuple[float, float]:
    """
    Run tokenizer on all GRS cases.
    Returns (mean_f1 * 100, total_seconds).
    """
    if not cases:
        return 0.0, 0.0
    scores = []
    t0 = time.perf_counter()
    for input_text, expected in cases:
        try:
            predicted = tokenize_fn(input_text)
        except Exception:
            predicted = []
        scores.append(f1_score(predicted, expected))
    elapsed = time.perf_counter() - t0
    return sum(scores) / len(scores) * 100, elapsed


# ---------------------------------------------------------------------------
# Shakespeare download
# ---------------------------------------------------------------------------


def ensure_shakespeare() -> Path:
    FIXTURES_DIR.mkdir(parents=True, exist_ok=True)
    if not SHAKESPEARE_FILE.exists():
        print(f"Downloading Shakespeare from {SHAKESPEARE_URL} ...", flush=True)
        urllib.request.urlretrieve(SHAKESPEARE_URL, SHAKESPEARE_FILE)
        size_mb = SHAKESPEARE_FILE.stat().st_size / 1024 / 1024
        print(f"Downloaded {size_mb:.2f} MB to {SHAKESPEARE_FILE}", flush=True)
    return SHAKESPEARE_FILE


# ---------------------------------------------------------------------------
# Tokenizer functions
# Each is a plain callable (str) -> list[str].
# Imports are module-level; only segmentation logic runs here.
# ---------------------------------------------------------------------------


def _tokenize_sentencex(text: str) -> list[str]:
    return list(_sentencex.segment("en", text))


_mw_tok = _MWTokenizer("en") if _MWTokenizer is not None else None


def _tokenize_mwtokenizer(text: str) -> list[str]:
    return list(_mw_tok.sentence_tokenize(text))  # type: ignore[union-attr]


def _tokenize_blingfire(text: str) -> list[str]:
    result = _blingfire.text_to_sentences(text)
    return result.splitlines() if result else []


def _tokenize_nltk(text: str) -> list[str]:
    return _nltk.sent_tokenize(text)


_pysbd_seg = (
    _pysbd.Segmenter(language="en", clean=False) if _pysbd is not None else None
)


def _tokenize_pysbd(text: str) -> list[str]:
    return _pysbd_seg.segment(text)  # type: ignore[union-attr]


def _tokenize_spacy(text: str) -> list[str]:
    doc = _spacy_nlp(text)
    return [sent.text for sent in doc.sents]


def _tokenize_spacy_dep(text: str) -> list[str]:
    doc = _spacy_nlp_full(text)
    return [sent.text for sent in doc.sents]


def _tokenize_syntok(text: str) -> list[str]:
    result = []
    for para in _syntok_segmenter.process(text):
        for sent in para:
            s = "".join(tok.spacing + tok.value for tok in sent).strip()
            if s:
                result.append(s)
    return result


_stanza_nlp = None


def _init_stanza() -> None:
    global _stanza_nlp
    if _stanza is None:
        return
    print("Downloading stanza 'en' model if needed...", flush=True)
    _stanza.download("en", processors="tokenize", verbose=False)
    _stanza_nlp = _stanza.Pipeline("en", processors="tokenize", verbose=False)


def _tokenize_stanza(text: str) -> list[str]:
    doc = _stanza_nlp(text)  # type: ignore[misc]
    return [sent.text for sent in doc.sentences]


# ---------------------------------------------------------------------------
# Library registry
# name, tokenize_fn, required_module, init_fn (optional)
# ---------------------------------------------------------------------------

LIBRARIES: list[
    tuple[str, Callable[[str], list[str]], object, Callable[[], None] | None]
] = [
    ("sentencex", _tokenize_sentencex, _sentencex, None),
    ("mwtokenizer", _tokenize_mwtokenizer, _MWTokenizer, None),
    ("blingfire", _tokenize_blingfire, _blingfire, None),
    ("nltk", _tokenize_nltk, _nltk, None),
    ("pysbd", _tokenize_pysbd, _pysbd, None),
    ("spacy", _tokenize_spacy, _spacy_nlp, None),
    ("spacy_dep", _tokenize_spacy_dep, _spacy_nlp_full, None),
    ("syntok", _tokenize_syntok, _syntok_segmenter, None),
    ("stanza", _tokenize_stanza, _stanza, _init_stanza),
]

# ---------------------------------------------------------------------------
# Benchmarking
# ---------------------------------------------------------------------------


def time_tokenizer(
    tokenize_fn: Callable[[str], list[str]],
    text: str,
    runs: int,
    timeout: float = 120.0,
) -> tuple[float, int]:
    """
    Return (avg_seconds, sentence_count).
    Stdout is suppressed during all runs (some libraries print progress).
    Total wall-clock is bounded by `timeout` seconds: the deadline is checked
    before each run, and if the previous run itself exceeded the budget the
    TimeoutError is raised before starting the next one.  This means a single
    slow run will complete but no further runs will be attempted.
    Returns the average over however many timed runs completed (minimum 1).
    """
    deadline = time.perf_counter() + timeout

    def run_silent() -> list[str]:
        with open(os.devnull, "w") as devnull:
            old_stdout = sys.stdout
            sys.stdout = devnull
            try:
                return tokenize_fn(text)
            finally:
                sys.stdout = old_stdout

    # warm-up — not timed, but counts toward the deadline
    result = run_silent()
    sentence_count = len(result)

    times = []
    for i in range(runs):
        if time.perf_counter() > deadline:
            if times:
                # already have data — report what we have rather than failing
                print(
                    f"  (timeout after {len(times)}/{runs} runs, "
                    f"reporting partial average)",
                    flush=True,
                )
                break
            raise TimeoutError(
                f"timed out before run 1 could start ({timeout:.0f}s limit)"
            )
        t0 = time.perf_counter()
        run_silent()
        times.append(time.perf_counter() - t0)

    return sum(times) / len(times), sentence_count


# ---------------------------------------------------------------------------
# Output formatting
# ---------------------------------------------------------------------------


def print_table(rows: list[dict], runs: int) -> None:
    col_name = max(len(r["name"]) for r in rows)
    col_name = max(col_name, len("Tokenizer"))

    header_speed = f"Speed (avg {runs} runs) s"

    print()
    print(
        f"| {'Tokenizer':<{col_name}} | {'GRS F1':>8} | {'GRS time (s)':>12} | {header_speed:>24} | {'Sentence Count':>14} |"
    )
    print(f"| {'-' * col_name} | {'-' * 8} | {'-' * 12} | {'-' * 24} | {'-' * 14} |")
    for r in rows:
        grs = f"{r['grs']:.2f}" if r["grs"] is not None else "N/A"
        grs_time = f"{r['grs_time']:.4f}" if r["grs_time"] is not None else "N/A"
        speed = f"{r['speed']:.4f}" if r["speed"] is not None else "N/A"
        sents = str(r["sents"]) if r["sents"] is not None else "N/A"
        print(
            f"| {r['name']:<{col_name}} | {grs:>8} | {grs_time:>12} | {speed:>24} | {sents:>14} |"
        )
    print()


def print_csv(rows: list[dict], runs: int) -> None:
    writer = csv.DictWriter(
        sys.stdout, fieldnames=["name", "grs", "grs_time", "speed", "sents"]
    )
    writer.writeheader()
    for r in rows:
        writer.writerow(
            {
                "name": r["name"],
                "grs": f"{r['grs']:.2f}" if r["grs"] is not None else "",
                "grs_time": f"{r['grs_time']:.4f}" if r["grs_time"] is not None else "",
                "speed": f"{r['speed']:.4f}" if r["speed"] is not None else "",
                "sents": r["sents"] if r["sents"] is not None else "",
            }
        )


def print_json(rows: list[dict], runs: int) -> None:
    print(json.dumps(rows, indent=2))


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Sentence segmenter benchmark: GRS F1 + Gutenberg speed",
    )
    parser.add_argument(
        "--runs",
        type=int,
        default=5,
        metavar="N",
        help="Number of timing runs per tokenizer (default: 5)",
    )
    parser.add_argument(
        "--speed-timeout",
        type=float,
        default=120.0,
        metavar="SECS",
        help="Per-tokenizer wall-clock timeout for speed benchmark in seconds (default: 120)",
    )
    parser.add_argument(
        "--include-stanza",
        action="store_true",
        help="Include stanza in the benchmark (~100s per run)",
    )
    parser.add_argument(
        "--grs-only",
        action="store_true",
        help="Run only the GRS scoring benchmark",
    )
    parser.add_argument(
        "--speed-only",
        action="store_true",
        help="Run only the Gutenberg speed benchmark",
    )
    parser.add_argument(
        "--output-format",
        choices=["table", "csv", "json"],
        default="table",
        help="Output format (default: table)",
    )
    args = parser.parse_args()

    run_grs = not args.speed_only
    run_speed = not args.grs_only

    # Load GRS cases
    grs_cases: list[tuple[str, list[str]]] = []
    if run_grs:
        if not GRS_FILE.exists():
            print(f"ERROR: GRS file not found: {GRS_FILE}", file=sys.stderr)
            sys.exit(1)
        grs_cases = load_grs(GRS_FILE)
        print(
            f"Loaded {len(grs_cases)} GRS test cases from {GRS_FILE.name}", flush=True
        )

    # Download Shakespeare
    shakespeare_text = ""
    if run_speed:
        shakespeare_path = ensure_shakespeare()
        shakespeare_text = shakespeare_path.read_text(
            encoding="utf-8", errors="replace"
        )
        size_mb = len(shakespeare_text.encode()) / 1024 / 1024
        print(
            f"Shakespeare corpus: {size_mb:.2f} MB, {args.runs} timing runs", flush=True
        )

    rows = []

    for name, tokenize_fn, required, init_fn in LIBRARIES:
        if name == "stanza" and not args.include_stanza:
            continue

        if required is None:
            print(f"\n[{name}] SKIP — import failed at startup", flush=True)
            rows.append(
                {
                    "name": name,
                    "grs": None,
                    "grs_time": None,
                    "speed": None,
                    "sents": None,
                }
            )
            continue

        print(f"\n[{name}]", flush=True)

        if init_fn is not None:
            try:
                init_fn()
            except Exception as exc:
                print(f"  SKIP — init failed: {exc}", flush=True)
                rows.append(
                    {
                        "name": name,
                        "grs": None,
                        "grs_time": None,
                        "speed": None,
                        "sents": None,
                    }
                )
                continue

        grs_score = None
        grs_time = None
        avg_speed = None
        sent_count = None

        if run_grs:
            print(f"  scoring GRS ({len(grs_cases)} cases)...", flush=True)
            try:
                grs_score, grs_time = score_grs(tokenize_fn, grs_cases)
                print(f"  GRS F1: {grs_score:.2f}  time: {grs_time:.4f}s", flush=True)
            except Exception as exc:
                print(f"  GRS failed: {exc}", flush=True)

        if run_speed:
            print(f"  timing {args.runs} runs on Shakespeare...", flush=True)
            try:
                avg_speed, sent_count = time_tokenizer(
                    tokenize_fn, shakespeare_text, args.runs, timeout=args.speed_timeout
                )
                print(f"  avg: {avg_speed:.4f}s, sentences: {sent_count}", flush=True)
            except TimeoutError as exc:
                print(f"  TIMEOUT: {exc}", flush=True)
            except Exception as exc:
                print(f"  speed benchmark failed: {exc}", flush=True)

        rows.append(
            {
                "name": name,
                "grs": grs_score,
                "grs_time": grs_time,
                "speed": avg_speed,
                "sents": sent_count,
            }
        )

    print("\n" + "=" * 60)
    print("Results")
    print("=" * 60)

    if args.output_format == "table":
        print_table(rows, args.runs)
    elif args.output_format == "csv":
        print_csv(rows, args.runs)
    elif args.output_format == "json":
        print_json(rows, args.runs)


if __name__ == "__main__":
    main()
