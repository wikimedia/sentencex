from sentencesegmenter.base import Language
from sentencesegmenter.terminators import GLOBAL_SENTENCE_TERMINATORS


class Armenian(Language):
    language = "hy"

    sentence_break_regex = r"[%s]+" % "".join(GLOBAL_SENTENCE_TERMINATORS + ["՜"])
