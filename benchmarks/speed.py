import time
from argparse import ArgumentParser, FileType

import blingfire
import nltk
import pysbd
import spacy
import stanza
import syntok.segmenter as syntok_segmenter
from mwtokenizer.tokenizer import Tokenizer as MWTokenizer
from syntok.tokenizer import Tokenizer

import sentencex

pysbd_segmenter = pysbd.Segmenter(language="en", clean=False, char_span=False)

nlp = spacy.blank("en")
nlp.add_pipe("sentencizer")
nlp_dep = spacy.load("en_core_web_sm", disable=["ner"])
stanza.download("en")
stanza_nlp = stanza.Pipeline(lang="en", processors="tokenize")

syntok_tokenizer = Tokenizer()
mwtokenizer = MWTokenizer(language_code="en")


def blingfire_tokenize(text):
    return blingfire.text_to_sentences(text).split("\n")


def nltk_tokenize(text):
    return nltk.sent_tokenize(text)


def pysbd_tokenize(text):
    segments = pysbd_segmenter.segment(text)
    segments = [s.strip() for s in segments]
    return segments


def spacy_tokenize(text):
    return [sent.text.strip("\n") for sent in nlp(text).sents]


def spacy_dep_tokenize(text):
    return [sent.text.strip("\n") for sent in nlp_dep(text).sents]


def stanza_tokenize(text):
    return [e.text for e in stanza_nlp(text).sentences]


def make_sentences(segmented_tokens):
    for sentence in segmented_tokens:
        yield "".join(str(token) for token in sentence).strip()


def syntok_tokenize(text):
    tokens = syntok_tokenizer.split(text)
    result = syntok_segmenter.split(iter(tokens))
    return list(make_sentences(result))


def mwtokenizer_tokenize(text):
    return list(mwtokenizer.sentence_tokenize(text, use_abbreviation=True))


def sentencex_segment(text):
    return list(sentencex.segment("en", text))


if __name__ == "__main__":
    parser = ArgumentParser(
        prog="benchmark", description="Measure sentence segmentation performance"
    )
    libraries = (
        sentencex_segment,
        mwtokenizer_tokenize,
        blingfire_tokenize,
        nltk_tokenize,
        pysbd_tokenize,
        spacy_tokenize,
        spacy_dep_tokenize,
        stanza_tokenize,
        syntok_tokenize,
    )

    parser.add_argument(
        "file",
        type=FileType("r"),
        help="Files to read, if empty, stdin is used",
    )

    args = parser.parse_args()
    big_text = args.file.read()
    print("Library\tSpeed\tSentences")
    for tokenize_func in libraries:
        start = time.time()
        sentences = tokenize_func(big_text)
        time_taken = time.time() - start
        print(f"{tokenize_func.__name__}\t{time_taken * 1000}s\t{len(sentences)}")

        # sentencesfile = open(f"benchmarks/{tokenize_func.__name__}.txt", "w")
        # snum = 0
        # for sentence in sentences:
        #     sentencesfile.write(f"{snum}:{sentence}\n")
        #     snum += 1
        # sentencesfile.close()
