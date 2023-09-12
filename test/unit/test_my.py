import pytest

from sentencex import segment

# ruff: noqa: E501

tests = [
    ("ခင္ဗ်ားနာမည္ဘယ္လိုေခၚလဲ။ င္ေနေကာင္းလား။", ["ခင္ဗ်ားနာမည္ဘယ္လိုေခၚလဲ။", "င္ေနေကာင္းလား။"])
]


@pytest.mark.parametrize("text,expected_sentences", tests)
def test_segment(text, expected_sentences):
    assert list(segment("my", text)) == expected_sentences
