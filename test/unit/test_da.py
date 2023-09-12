import pytest

from sentencex import segment

# ruff: noqa: E501

tests = [
    ("Hej Verden. Mit navn er Jonas.", ["Hej Verden.", "Mit navn er Jonas."]),
    ("Hvad er dit navn? Mit nav er Jonas.", ["Hvad er dit navn?", "Mit nav er Jonas."]),
    pytest.param(
        "Lad os spørge Jane og co. De burde vide det.",
        ["Lad os spørge Jane og co.", "De burde vide det."],
        marks=pytest.mark.xfail,
    ),
    pytest.param(
        "De lukkede aftalen med Pitt, Briggs & Co. Det lukkede i går.",
        ["De lukkede aftalen med Pitt, Briggs & Co.", "Det lukkede i går."],
        marks=pytest.mark.xfail,
    ),
    ("De holdt Skt. Hans i byen.", ["De holdt Skt. Hans i byen."]),
    (
        "St. Michael's Kirke er på 5. gade nær ved lyset.",
        ["St. Michael's Kirke er på 5. gade nær ved lyset."],
    ),
]


@pytest.mark.parametrize("text,expected_sentences", tests)
def test_segment(text, expected_sentences):
    assert list(segment("da", text)) == expected_sentences
