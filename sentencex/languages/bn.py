from sentencex.base import Language

from .en import English


class Bengali(Language):
    language = "bn"

    # Writing English abbreviation like Dr as such inside Bengali is common
    abbreviations = English.abbreviations.union(
        {
            "এ",
            "বি",
            "সি",
            "ডি",
            "ঈ",
            "এফ",
            "জি",
            "এইচ",
            "আই",
            "জে",
            "কে",
            "এল",
            "এম",
            "এন",
            "ও",
            "পি",
            "কিউ",
            "আর",
            "এস",
            "টি",
            "ইউ",
            "ভি",
            "ডাবলিউ",
            "এক্স",
            "ওয়াই",
            "জেড",
        }
    )
