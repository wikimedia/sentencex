import regex

from sentencex.base import Language


class Armenian(Language):
    language = "hy"

    # Don't break on "."
    # Do break on "։", "՜" and ":"
    sentence_break_regex = regex.compile(r"(?:[^\P{SentenceTerminal}.]|[։՜:])+")
