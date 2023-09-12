import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    (
        "सच्चाई यह है कि इसे कोई नहीं जानता। हो सकता है यह फ़्रेन्को के खिलाफ़ कोई विद्रोह रहा हो, या फिर बेकाबू हो गया कोई आनंदोत्सव।",
        [
            "सच्चाई यह है कि इसे कोई नहीं जानता।",
            "हो सकता है यह फ़्रेन्को के खिलाफ़ कोई विद्रोह रहा हो, या फिर बेकाबू हो गया कोई आनंदोत्सव।",
        ],
    )
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("hi", text)) == expected_sents
