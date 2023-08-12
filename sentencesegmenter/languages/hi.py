from sentencesegmenter.base import Language

from .en import English


class Hindi(Language):
    language = "hi"

    # Writing English abbreviation like Dr as such inside Hindi is common
    abbreviations = English.abbreviations.union({"पी"})
