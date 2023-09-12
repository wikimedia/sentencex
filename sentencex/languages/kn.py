from sentencex.base import Language

from .en import English


class Kannada(Language):
    language = "kn"
    # Writing English abbreviation like Dr as such inside Kannada is common
    abbreviations = English.abbreviations.union(
        {
            "ಎ",
            "ಬಿ",
            "ಸಿ",
            "ಡಿ",
            "ಈ",
            "ಎಫ್",
            "ಜಿ",
            "ಹೆಚ್",
            "ಐ",
            "ಜೆ",
            "ಕೆ",
            "ಎಲ್",
            "ಎಂ",
            "ಎನ್",
            "ಓ",
            "ಪಿ",
            "ಕ್ಯೂ",
            "ಆರ್",
            "ಎಸ್",
            "ಟಿ",
            "ಯೂ",
            "ವಿ",
            "ಡಬಲ್ಯೂ",
            "ಎಕ್ಸ್",
            "ವೈ",
            "ಜೆಡ್",
        }
    )
