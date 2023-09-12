import pytest

from sentencex import segment

tests = [
    (
        "እንደምን አለህ፧መልካም ቀን ይሁንልህ።እባክሽ ያልሽዉን ድገሚልኝ።",
        ["እንደምን አለህ፧", "መልካም ቀን ይሁንልህ።", "እባክሽ ያልሽዉን ድገሚልኝ።"],
    ),
]


@pytest.mark.parametrize("text,expected_sentences", tests)
def test_segment(text, expected_sentences):
    assert list(segment("am", text)) == expected_sentences
