import re

from sentencex.base import Language
from sentencex.terminators import GLOBAL_SENTENCE_TERMINATORS


class Greek(Language):
    language = "el"

    sentence_break_regex = re.compile(r"[%s]+" % "".join(GLOBAL_SENTENCE_TERMINATORS + [";"]))
