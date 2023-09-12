import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    (
        "Ide o majiteľov firmy ABTrade s. r. o., ktorí stoja aj za ďalšími spoločnosťami, napr. XYZCorp a.s.",
        [
            "Ide o majiteľov firmy ABTrade s. r. o., ktorí stoja aj za ďalšími spoločnosťami, napr. XYZCorp a.s."
        ],
    ),
    (
        "„Prieskumy beriem na ľahkú váhu. V podstate ma to nezaujíma,“ reagoval Matovič na prieskum agentúry Focus.",
        [
            "„Prieskumy beriem na ľahkú váhu. V podstate ma to nezaujíma,“ reagoval Matovič na prieskum agentúry Focus."
        ],
    ),
    (
        "Toto sa mi podarilo až na 10. pokus, ale stálo to za to.",
        ["Toto sa mi podarilo až na 10. pokus, ale stálo to za to."],
    ),
    (
        "Ide o príslušníkov XII. Pluku špeciálneho určenia.",
        ["Ide o príslušníkov XII. Pluku špeciálneho určenia."],
    ),
    (
        "Spoločnosť bola založená 7. Apríla 2020, na zmluve však figuruje dátum 20. marec 2020.",
        ["Spoločnosť bola založená 7. Apríla 2020, na zmluve však figuruje dátum 20. marec 2020."],
    ),
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("sk", text)) == expected_sents
