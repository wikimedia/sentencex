import pytest

from sentencex import segment

tests = [
    (
        "እንደምን አለህ፧መልካም ቀን ይሁንልህ።እባክሽ ያልሽዉን ድገሚልኝ።",
        ["እንደምን አለህ፧", "መልካም ቀን ይሁንልህ።", "እባክሽ ያልሽዉን ድገሚልኝ።"],
    ),
    (
        "ቴዎድሮስ ጥር ፮ ቀን ፲፰፻፲፩ ዓ.ም. ሻርጌ በተባለ ቦታ ቋራ ውስጥ፣ ከጎንደር ከተማ በስተ ምዕራብ ተወለዱ።",
        ["ቴዎድሮስ ጥር ፮ ቀን ፲፰፻፲፩ ዓ.ም. ሻርጌ በተባለ ቦታ ቋራ ውስጥ፣ ከጎንደር ከተማ በስተ ምዕራብ ተወለዱ።"],
    ),
]


@pytest.mark.parametrize("text,expected_sentences", tests)
def test_segment(text, expected_sentences):
    assert list(segment("am", text)) == expected_sentences
