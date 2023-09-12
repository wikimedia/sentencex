from sentencex.base import Language

from .en import English


class Odia(Language):
    language = "or"

    # Writing English abbreviation like Dr as such inside Odia is common
    abbreviations = English.abbreviations.union(
        {
            "ଏ",
            "ବି",
            "ସି",
            "ଡି",
            "ଈ",
            "ଏଫ",
            "ଜି",
            "ହ୍",
            "ଆଇ",
            "ଜେ",
            "କେ",
            "ଏଲ",
            "ଏମ",
            "ଏନ",
            "ଓ",
            "ପି",
            "କ୍ୟୁ",
            "ଆର",
            "ଏସ",
            "ଟି",
            "ୟୁ",
            "ଭି",
            "ଡବଲ୍ୱ୍",
            "ଏକ୍ସ",
            "ଏବଂ",
            "ଜେଡ",
        }
    )
