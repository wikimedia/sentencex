import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    ("To słowo bałt. jestskrótem.", ["To słowo bałt. jestskrótem."]),
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("pl", text)) == expected_sents
