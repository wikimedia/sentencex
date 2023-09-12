from sentencex.base import Language

from .en import English


class Punjabi(Language):
    language = "pa"
    # Writing English abbreviation like Dr as such inside Punjabi is common
    abbreviations = English.abbreviations.union(
        {
            "ਏ",
            "ਬੀ",
            "ਸੀ",
            "ਡੀ",
            "ਈ",
            "ਐਫ",
            "ਜੀ",
            "ਐਚ",
            "ਆਈ",
            "ਜੇ",
            "ਕੇ",
            "ਐਲ",
            "ਐਮ",
            "ਐਨ",
            "ਓ",
            "ਪੀ",
            "ਕਿਊ",
            "ਆਰ",
            "ਐਸ",
            "ਟੀ",
            "ਯੂ",
            "ਵੀ",
            "ਡਬਲਯੂ",
            "ਐਕਸ",
            "ਵਾਈ",
            "ਜੇਡ",
        }
    )
