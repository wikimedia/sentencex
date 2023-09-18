import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    (
        "Se julkaistiin singlenä 7. heinäkuuta 1997, ja se nousi listaykköseksi yhtyeen kotimaassa Britanniassa sekä Irlannissa, Suomessa, Espanjassa ja Kanadassa",
        [
            "Se julkaistiin singlenä 7. heinäkuuta 1997, ja se nousi listaykköseksi yhtyeen kotimaassa Britanniassa sekä Irlannissa, Suomessa, Espanjassa ja Kanadassa"
        ],
    ),
    (
        "Brittiläinen musiikkilehti NME valitsi lokakuussa 2011 ”D’You Know What I Meanin?” sijalle 77 listallaan, joka sisälsi 150 parasta kappaletta vuosilta 1996–2011.",
        [
            "Brittiläinen musiikkilehti NME valitsi lokakuussa 2011 ”D’You Know What I Meanin?” sijalle 77 listallaan, joka sisälsi 150 parasta kappaletta vuosilta 1996–2011."
        ],
    ),
    (
        "Netistä ladattu musiikki on otettu huomioon singlelistalla 3. lokakuuta 2007 lähtien.[13] Uudistus muutti listan luonnetta.",
        [
            "Netistä ladattu musiikki on otettu huomioon singlelistalla 3. lokakuuta 2007 lähtien.[13]",
            "Uudistus muutti listan luonnetta.",
        ],
    ),
    (
        "Radiomafia oli Yleisradion pääasiassa nuorille ja nuorille aikuisille suunnattu radiokanava, joka aloitti toimintansa vuoden 1990 radiouudistuksessa 1. kesäkuuta 1990 ja lopetti Ylen radiouudistuksen myötä 12. tammikuuta 2003.",
        [
            "Radiomafia oli Yleisradion pääasiassa nuorille ja nuorille aikuisille suunnattu radiokanava, joka aloitti toimintansa vuoden 1990 radiouudistuksessa 1. kesäkuuta 1990 ja lopetti Ylen radiouudistuksen myötä 12. tammikuuta 2003.",
        ],
    ),
    (
        "Dr. Alban (oikealta nimeltä Alban Uzoma Nwapa, s. 26. elokuuta 1957 Oguta, Brittiläinen Nigeria) on nigerialaissyntyinen ruotsalainen eurodance/rap/reggae -artisti.",
        [
            "Dr. Alban (oikealta nimeltä Alban Uzoma Nwapa, s. 26. elokuuta 1957 Oguta, Brittiläinen Nigeria) on nigerialaissyntyinen ruotsalainen eurodance/rap/reggae -artisti.",
        ],
    ),
]


@pytest.mark.parametrize("text,expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("fi", text)) == expected_sents
