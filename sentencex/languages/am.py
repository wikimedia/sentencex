from sentencex.base import Language

from .en import English


class Amharic(Language):
    language = "am"

    abbreviations = English.abbreviations.union({"ዓ", "ም"})
