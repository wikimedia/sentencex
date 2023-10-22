import regex

from sentencex.base import Language


class Greek(Language):
    language = "el"

    sentence_break_regex = regex.compile(r"[\p{Sentence_Terminal};]+")
