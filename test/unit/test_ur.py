import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    (
        "کیا حال ہے؟ ميرا نام ___ ەے۔ میں حالا تاوان دےدوں؟",
        ["کیا حال ہے؟", "ميرا نام ___ ەے۔", "میں حالا تاوان دےدوں؟"],
    ),
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("ur", text)) == expected_sents
