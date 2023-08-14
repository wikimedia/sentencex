import re
from typing import Dict, List

from .terminators import GLOBAL_SENTENCE_TERMINATORS

GLOBAL_SENTENCE_BOUNDARY_REGEX = r"[%s]+" % "".join(GLOBAL_SENTENCE_TERMINATORS)
QUOTES_REGEX = r"([\"'«‘‚‛“„‟‹《])(?:\\\1|.)*?[\"'»’‚‛”„‟›》]"
EMAIL_REGEX = r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,7}"

NUMBERED_REFERENCE_REGEX = r"^(\[\d+])+"


class Languages(type):
    REGISTRY: Dict[str, type] = {}

    def __init__(cls, name, bases, attrs):
        """
        Here the name of the class is used as key but it could be any class
        parameter.
        """
        if name != "Language":
            Languages.REGISTRY[cls.language] = cls

    @classmethod
    def get_registry(cls) -> Dict[str, type]:
        return cls.REGISTRY


class Language(object, metaclass=Languages):
    """
    Any class that will inherits from BaseAbbrevRegisteredClass will be included
    inside the dict AbbrevRegistryHolder.REGISTRY, the key being the name of the
    class and the associated value, the class itself.
    """

    language = "base"
    abbreviations: set = set()
    sentence_break_regex = GLOBAL_SENTENCE_BOUNDARY_REGEX

    def is_abbreviation(self, head: str, tail: str) -> bool:
        """
        Do not break in abbreviations. Example D. John, St. Peter
        In the case of "This is Dr. Watson", head is "This is Dr", tail is " Watson"
        """
        lastword = self.get_lastword(head)

        if len(lastword) == 0:
            return False

        is_abbrev = (
            lastword in self.abbreviations
            or (lastword[0].lower() + lastword[1:] in self.abbreviations)
            or (lastword.upper() in self.abbreviations)
        )

        return is_abbrev

    def get_lastword(self, text: str):
        return re.split(r"[\s\.]+", text)[-1]

    def findBoundary(self, text, match):
        tail = text[match.start() + 1 :]
        head = text[: match.start()]

        # Trailing non-final punctuation: not a sentence boundary
        # if re.match(r"^[,;:]", tail):
        #     return None

        # If next word is numbered reference, expand boundary to that.'
        number_ref_match = re.match(NUMBERED_REFERENCE_REGEX, tail)

        if number_ref_match:
            return match.start() + 1 + len(number_ref_match.group(0))

        # Next character is number or lower-case: not a sentence boundary
        if re.match(r"^\W*[0-9a-z]", tail):
            return None
        if self.is_abbreviation(head, tail):
            return None
        # Include any closing punctuation and trailing space
        match_len = len(match.group(0))
        # print(match_len)
        return match.start() + match_len

    def segment(self, text: str) -> List[str]:
        sentences = []
        paragraph_break = "\n\n"
        paragraphs = text.split(paragraph_break)
        paragraph_index = 0
        for paragraph in paragraphs:
            skippable_ranges = []
            if paragraph_index > 0:
                sentences.append(paragraph_break)
            boundaries = [0]
            quote_matches = re.finditer(QUOTES_REGEX, paragraph)

            for quote_match in quote_matches:
                skippable_ranges.append(quote_match.span())

            email_matches = re.finditer(EMAIL_REGEX, paragraph)
            for quote_match in email_matches:
                skippable_ranges.append(quote_match.span())

            matches = re.finditer(self.sentence_break_regex, paragraph)

            for match in matches:
                boundary = self.findBoundary(paragraph, match)

                if boundary is None:
                    continue

                # Skip breaks that are inside a quote.
                in_range = False
                for qstart, qend in skippable_ranges:
                    print(
                        boundary,
                        qstart,
                        qend,
                    )
                    if boundary > qstart and boundary < qend:
                        in_range = True
                        break
                if in_range:
                    continue

                boundaries.append(boundary)

            for i, j in zip(boundaries, boundaries[1:] + [None]):
                sentence = paragraph[i:j]
                if len(sentence):
                    sentence = sentence.strip(" ")
                    sentences.append(sentence)
            paragraph_index += 1

        return sentences
