import regex

from sentencex.base import Language


class Burmese(Language):
    language = "my"

    # See https://en.wiktionary.org/wiki/၏
    sentence_break_regex = regex.compile(r"[\p{Sentence_Terminal}၏]+")
    abbreviations = {}
