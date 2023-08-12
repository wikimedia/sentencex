import pytest

from sentencesegmenter import segment

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
    # assert len(segment("el", text)) <= len(expected_sents)
    assert segment("el", text) == expected_sents
