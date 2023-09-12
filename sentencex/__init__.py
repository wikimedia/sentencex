from typing import List

from . import languages
from .base import Language, Languages
from .fallbacks import LANGUAGE_FALLBACKS


def get_language_class(language: str) -> Language:
    if language in Languages.REGISTRY:
        return Languages.REGISTRY[language]

    fallbacks = LANGUAGE_FALLBACKS.get(language, ["en"])
    for fallback_language in fallbacks:
        cls = get_language_class(fallback_language)
        if cls:
            return cls


def segment(language, text: str) -> List[str]:
    """
    Segments the given text into sentences based on the specified language.

    Parameters:
        language (str): The language identifier, in ISO 639-1 format.
        text (str): The input text to be segmented.

    Returns:
        List[str]: A list of sentences.
    """
    return get_language_class(language)().segment(text)


__all__ = ["languages", "segment"]
