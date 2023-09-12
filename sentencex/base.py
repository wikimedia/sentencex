import re
from typing import Dict, Iterator, Tuple

from .terminators import GLOBAL_SENTENCE_TERMINATORS


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

    EXCLAMATION_WORDS = set(
        (
            "!Xũ !Kung ǃʼOǃKung !Xuun !Kung-Ekoka ǃHu ǃKhung ǃKu ǃung ǃXo ǃXû ǃXung "
            + "ǃXũ !Xun Yahoo! Y!J Yum!"
        ).split()
    )

    numbered_reference_regex = re.compile(r"^(\[\d+])+")
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

    def is_exclamation_word(self, head: str, tail: str) -> bool:
        lastword = self.get_lastword(head)
        return lastword + "!" in self.EXCLAMATION_WORDS

    def get_lastword(self, text: str):
        return re.split(r"[\s\.]+", text)[-1]

    def findBoundary(self, text, match):
        tail = text[match.start() + 1 :]
        head = text[: match.start()]

        # Trailing non-final punctuation: not a sentence boundary
        # if re.match(r"^[,;:]", tail):
        #     return None

        # If next word is numbered reference, expand boundary to that.'
        number_ref_match = self.numbered_reference_regex.match(tail)

        if number_ref_match:
            return match.start() + 1 + len(number_ref_match.group(0))

        # Next character is number or lower-case: not a sentence boundary
        if self.continue_in_next_word(tail):
            return None
        if self.is_abbreviation(head, tail):
            return None
        if self.is_exclamation_word(head, tail):
            return None

        # Include any closing punctuation and trailing space
        match_len = len(match.group(0))
        # print(match_len)
        return match.start() + match_len

    def continue_in_next_word(self, text_after_boundary) -> bool:
        return re.match(r"^[0-9a-z]", text_after_boundary)

    def get_skippable_ranges(self, text) -> Tuple[int, int]:
        # Create a list of skippable ranges, such as quotes, parentheses, and email addresses.
        skippable_ranges = [match.span() for match in self.quotes_regex.finditer(text)]
        skippable_ranges += [match.span() for match in self.parens_regex.finditer(text)]
        skippable_ranges += [match.span() for match in self.email_regex.finditer(text)]
        return skippable_ranges

    def segment(self, text: str) -> Iterator[str]:
        """
        Splits the given input text into sentences.

        Args:
            text (str): The input text to be segmented into sentences.

        Yields:
            Iterator[str]: An iterator that yields each sentence from the input text.

        """
        # Split the text into paragraphs using consecutive newlines as delimiters.
        paragraphs = re.split(r"(\n{2,})", text)

        # Iterate over each paragraph.
        for paragraph in paragraphs:
            # Initialize a list to store the boundaries of sentences.
            boundaries = [0]

            # Find all matches of sentence breaks in the paragraph.
            matches = self.sentence_break_regex.finditer(paragraph)
            skippable_ranges = self.get_skippable_ranges(paragraph)

            # Iterate over each match of sentence breaks.
            for match in matches:
                # Find the boundary of the sentence.
                boundary = self.findBoundary(paragraph, match)

                # If boundary is None, skip to the next match.
                if boundary is None:
                    continue

                # Check if the boundary is inside a skippable range (quote, parentheses, or email).
                in_range = False
                for qstart, qend in skippable_ranges:
                    if boundary > qstart and boundary < qend:
                        if boundary + 1 == qend and self.is_punctuation_between_quotes():
                            boundary = qend
                            in_range = False
                        else:
                            in_range = True
                        break

                # If in_range is True, skip to the next match.
                if in_range:
                    continue

                # Add the boundary to the boundaries list.
                boundaries.append(boundary)

            # Iterate over each pair of boundaries.
            for i, j in zip(boundaries, boundaries[1:] + [None]):
                # Slice the paragraph using the boundaries to get the sentence.
                sentence = paragraph[i:j]

                # If the sentence has a length, yield the sentence
                # stripped of leading/trailing spaces.
                if len(sentence):
                    yield sentence.strip(" ")

    def is_punctuation_between_quotes(self) -> bool:
        return False
