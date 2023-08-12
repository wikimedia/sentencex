from sentencesegmenter.base import Language

from .en import English


class Kannada(Language):
    language = "kn"
    # Writing English abbreviation like Dr as such inside Kannada is common
    abbreviations = English.abbreviations.union(
        {
            # TODO: add language abbreviations here
        }
    )
