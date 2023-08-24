import re

from sentencesegmenter.base import Language
from sentencesegmenter.terminators import GLOBAL_SENTENCE_TERMINATORS


class Armenian(Language):
    language = "hy"

    hy_terminators = GLOBAL_SENTENCE_TERMINATORS + ["։", "՜", ":"]
    hy_terminators.remove(".")
    hy_terminators.remove("...")
    sentence_break_regex = re.compile(r"[%s]+" % "".join(hy_terminators))
