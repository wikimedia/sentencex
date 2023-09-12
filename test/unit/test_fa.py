import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    (
        "خوشبختم، آقای رضا. شما کجایی هستید؟ من از تهران هستم.",
        ["خوشبختم، آقای رضا.", "شما کجایی هستید؟", "من از تهران هستم."],
    )
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("mr", text)) == expected_sents
