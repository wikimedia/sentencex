import pytest

from sentencex import get_language_class
from sentencex.fallbacks import LANGUAGE_FALLBACKS


@pytest.mark.parametrize("language", list(LANGUAGE_FALLBACKS))
def test_fallback(language):
    fallback_class = get_language_class(language)
    assert fallback_class
