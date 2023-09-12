from sentencex.base import Language

from .en import English


class Telugu(Language):
    language = "te"

    # Writing English abbreviation like Dr as such inside Telugu is common
    abbreviations = English.abbreviations.union(
        {
            "ఎ",
            "బి",
            "సి",
            "డి",
            "ఈ",
            "ఎఫ్",
            "జి",
            "హెచ్",
            "ఐ",
            "జె",
            "కె",
            "ఎల్",
            "ఎం",
            "ఎన్",
            "ఓ",
            "పి",
            "క్యూ",
            "ఆర్",
            "ఎస్",
            "టి",
            "యూ",
            "వి",
            "డబ్ల్యూ",
            "ఎక్స్",
            "వై",
            "జెడ్",
        }
    )
