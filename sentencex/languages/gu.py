from sentencex.base import Language

from .en import English


class Gujarati(Language):
    language = "gu"

    # Writing English abbreviation like Dr as such inside Tamil is common
    abbreviations = English.abbreviations.union(
        {
            "એ",
            "બી",
            "સી",
            "ડી",
            "ઈ",
            "એફ",
            "જી",
            "એચ",
            "આઈ",
            "જે",
            "કે",
            "એલ",
            "એમ",
            "એન",
            "ઓ",
            "પી",
            "ક્યૂ",
            "આર",
            "એસ",
            "ટી",
            "યૂ",
            "વી",
            "ડબલ્યૂ",
            "એક્સ",
            "વાય",
            "જેડ",
        }
    )
