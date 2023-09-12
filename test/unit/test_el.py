import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    (
        "Με συγχωρείτε· πού είναι οι τουαλέτες; Τις Κυριακές δε δούλευε κανένας. το κόστος του σπιτιού ήταν £260.950,00.",
        [
            "Με συγχωρείτε· πού είναι οι τουαλέτες;",
            "Τις Κυριακές δε δούλευε κανένας.",
            "το κόστος του σπιτιού ήταν £260.950,00.",
        ],
    ),
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("el", text)) == expected_sents
