import pytest

from sentencex import segment

# ruff: noqa: E501

tests = [
    (
        "Hij schoot op de JP8-brandstof toen de Surface-to-Air (sam)-missiles op hem af kwamen. 81 procent van de schoten was raak.",
        [
            "Hij schoot op de JP8-brandstof toen de Surface-to-Air (sam)-missiles op hem af kwamen.",
            "81 procent van de schoten was raak.",
        ],
    ),
    (
        "81 procent van de schoten was raak. ...en toen barste de hel los.",
        ["81 procent van de schoten was raak.", "...", "en toen barste de hel los."],
    ),
    ("Afkorting aanw. vnw.", ["Afkorting aanw. vnw."]),
]


@pytest.mark.parametrize("text,expected_sentences", tests)
def test_segment(text, expected_sentences):
    assert list(segment("nl", text)) == expected_sentences
