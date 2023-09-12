from sentencex.base import Language

from .en import English


class Hindi(Language):
    language = "hi"

    # Writing English abbreviation like Dr as such inside Hindi is common
    abbreviations = English.abbreviations.union(
        {
            "ए",
            "बी",
            "सी",
            "डी",
            "ई",
            "एफ",
            "जी",
            "एच",
            "आई",
            "जे",
            "के",
            "एल",
            "एम",
            "एन",
            "ओ",
            "पी",
            "क्यू",
            "आर",
            "एस",
            "टी",
            "यू",
            "भी",
            "डब्लू",
            "एक्स",
            "वाई",
            "जेड",
        }
    )
