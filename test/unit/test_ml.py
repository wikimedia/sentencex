import pytest

from sentencex import segment

tests = [
    ("ഇത് ഡോ. ശിവൻ. ഇദ്ദേഹമാണ് ഞാൻ പറഞ്ഞയാൾ", ["ഇത് ഡോ. ശിവൻ.", "ഇദ്ദേഹമാണ് ഞാൻ പറഞ്ഞയാൾ"]),
    ("ഇത് മി. കെ. പി. മോഹനൻ", ["ഇത് മി. കെ. പി. മോഹനൻ"]),
    ("ഇത് പ്രൊ. കെ.പി. മോഹനൻ", ["ഇത് പ്രൊ. കെ.പി. മോഹനൻ"]),
    ("ഇത് Dr. മോഹനൻ", ["ഇത് Dr. മോഹനൻ"]),
]


@pytest.mark.parametrize("text, expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("ml", text)) == expected_sents
