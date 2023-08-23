import re

from sentencesegmenter.base import Language
from sentencesegmenter.terminators import GLOBAL_SENTENCE_TERMINATORS


class Burmese(Language):
    language = "my"

    # See https://en.wiktionary.org/wiki/၏
    sentence_break_regex = re.compile(r"[%s]+" % "".join(GLOBAL_SENTENCE_TERMINATORS + ["၏"]))
    abbreviations = {}
