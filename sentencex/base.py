import re
from dataclasses import dataclass, field
from typing import Dict, Iterator, List, Tuple

from .terminators import GLOBAL_SENTENCE_TERMINATORS


@dataclass
class SentenceBoundary:
    """Class for keeping track of a sentence boundary."""

    start: int = 0  # The start of the boundary region
    end: int = 0  # The end of the boundary region
    term_index: int = 0  # The index of the terminator
    terminator: str = ""  # Terminator character
    ambiguous: bool = field(init=False)

    def __post_init__(self):
        self.ambiguous = self.is_ambiguous()

    def is_ambiguous(self) -> bool:
        """whether the sentence terminating punctuation is ambiguous"""
        return (
            self.terminator
            in [
                "\u002E"  # FULL STOP
                "\u2024"  # ONE DOT LEADER
                "\uFE52"  # SMALL FULL STOP
                "\uFF0E"  # FULLWIDTH FULL STOP
            ]
        )

    def length(self):
        return len(self.end - self.start)

    def apply_offset(self, offset: int):
        self.start += offset
        self.end += offset
        self.term_index += offset

    def get_sentence(self, text):
        return text[self.start : self.end]


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

    quote_pairs = {
        '"': '"',
        " '": "'",  # Need a space before ' to avoid capturing don't , l'Avv etc
        "«": "»",
        "‘": "’",
        "‚": "‚",
        "“": "”",
        "‛": "‛",
        "„": "“",
        "‟": "‟",
        "‹": "›",
        "《": "》",
        "「": "」",
    }

    language = "base"
    abbreviations: set = set()
    GLOBAL_SENTENCE_BOUNDARY_REGEX = re.compile(r"[%s]+" % "".join(GLOBAL_SENTENCE_TERMINATORS))
    quotes_regx_str = r"|".join([f"{left}(\n|.)*?{right}" for left, right in quote_pairs.items()])
    quotes_regex = re.compile(r"%s+" % quotes_regx_str)
    parens_regex = re.compile(r"([\(（<{\[])(?:\\\1|.)*?[\)\]}）]")
    email_regex = re.compile(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,7}")
    abbreviation_char = "."
    EXCLAMATION_WORDS = set(
        (
            "!Xũ !Kung ǃʼOǃKung !Xuun !Kung-Ekoka ǃHu ǃKhung ǃKu ǃung ǃXo ǃXû ǃXung "
            + "ǃXũ !Xun Yahoo! Y!J Yum!"
        ).split()
    )

    numbered_reference_regex = re.compile(r"^(\[\d+])+")
    sentence_break_regex = GLOBAL_SENTENCE_BOUNDARY_REGEX

    def is_abbreviation(self, head: str, tail: str, seperator: str) -> bool:
        """
        Do not break in abbreviations. Example D. John, St. Peter
        In the case of "This is Dr. Watson", head is "This is Dr", tail is " Watson"
        """

        if self.abbreviation_char != seperator:
            return False

        lastword = self.get_lastword(head)

        if len(lastword) == 0:
            return False

        is_abbrev = (
            lastword in self.abbreviations
            or (lastword[0].lower() + lastword[1:] in self.abbreviations)
            or (lastword.upper() in self.abbreviations)
        )

        return is_abbrev

    def is_exclamation_word(self, head: str, tail: str) -> bool:
        lastword = self.get_lastword(head)
        return lastword + "!" in self.EXCLAMATION_WORDS

    def get_lastword(self, text: str):
        return re.split(r"[\s\.]+", text)[-1]

    def findBoundary(self, text, match) -> SentenceBoundary:
        [match_start_index, match_end_index] = match.span()
        end = match_end_index

        tail = text[match_start_index + 1 :]
        head = text[:match_start_index]

        # If next word is numbered reference, expand boundary to that.'
        number_ref_match = self.numbered_reference_regex.match(tail)

        if number_ref_match:
            end = match_end_index + len(number_ref_match.group(0))

        # Next character is number or lower-case: not a sentence boundary
        if self.continue_in_next_word(tail) and not number_ref_match:
            return None
        if self.is_abbreviation(head, tail, match.group(0)):
            return None
        if self.is_exclamation_word(head, tail):
            return None

        continuing_white_spaces = re.match(r"^\s+", tail)
        if continuing_white_spaces:
            end = end + len(continuing_white_spaces.group(0))

        return SentenceBoundary(term_index=match_start_index, end=end, terminator=match.group(0))

    def continue_in_next_word(self, text_after_boundary) -> bool:
        return re.match(r"^[0-9a-z]", text_after_boundary)

    def get_skippable_ranges(self, text) -> Tuple[int, int]:
        # Create a list of skippable ranges, such as quotes, parentheses, and email addresses.
        skippable_ranges = [match.span() for match in self.quotes_regex.finditer(text)]
        skippable_ranges += [match.span() for match in self.parens_regex.finditer(text)]
        skippable_ranges += [match.span() for match in self.email_regex.finditer(text)]
        return skippable_ranges

    def get_boundaries(self, text: str) -> Iterator[SentenceBoundary]:
        """
        Get sentence boundaries in the given input text.

        Args:
            text (str): The input text to be segmented into sentences.

        Yields:
            Iterator[SentenceBoundary]: An iterator that yields `SentenceBoundary`.

        """

        # Split the text into paragraphs using consecutive newlines as delimiters.
        # The result will have the delimiter as member of array.
        paragraphs: List[str] = re.split(r"([\n]{2})", text)

        # paragraph offset
        offset: int = 0

        paragraph: str
        for paragraph in paragraphs:
            # Find all matches of sentence breaks in the paragraph.
            matches = self.sentence_break_regex.finditer(paragraph)
            skippable_ranges = self.get_skippable_ranges(paragraph)

            prev_end = offset
            # Iterate over each match of sentence breaks.
            for match in matches:
                boundary: SentenceBoundary = self.findBoundary(paragraph, match)

                # If boundary is None, skip to the next match.
                if boundary is None:
                    continue
                boundary.start = prev_end

                # Check if the boundary is inside a skippable range (quote, parentheses, or email).
                in_range = False
                for qstart, qend in skippable_ranges:
                    if boundary.end > qstart and boundary.end < qend:
                        if boundary.end + 1 == qend and self.is_punctuation_between_quotes():
                            boundary.end = qend
                            boundary.close = paragraph[qend - 1]
                            in_range = False
                        else:
                            in_range = True
                        break

                # If in_range is True, skip to the next match.
                if in_range:
                    continue

                # Add the boundary to the boundaries list.
                boundary.apply_offset(offset)
                yield boundary
                prev_end = boundary.end

            if prev_end != len(paragraph):
                yield SentenceBoundary(start=prev_end, end=len(paragraph) + offset)

            offset += len(paragraph)

    def segment(self, text: str) -> Iterator[str]:
        """
        Splits the given input text into sentences.

        Args:
            text (str): The input text to be segmented into sentences.

        Yields:
            Iterator[str]: An iterator that yields each sentence from the input text.

        """
        boundaries = self.get_boundaries(text)
        for boundary in boundaries:
            yield boundary.get_sentence(text).strip(" ")

    def is_punctuation_between_quotes(self) -> bool:
        return False
