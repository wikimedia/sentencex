import time
from argparse import ArgumentParser

import blingfire
import nltk
import pysbd
import spacy
import stanza
import syntok.segmenter as syntok_segmenter
from en_golden_rules import GOLDEN_EN_RULES
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


total_rules = len(GOLDEN_EN_RULES)


def benchmark(golden_rules, tokenize_func):
    score = 0
    for rule in golden_rules:
        text, expected = rule
        segments = tokenize_func(text)
        if segments == expected:
            score += 1
    percent_score = (score / total_rules) * 100.0

    return percent_score


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
    print("{:30}{:10}{:30}".format("Tokenizer", "GRS score", "Speed(Avg over 100 runs)"))
    for tokenize_func in libraries:
        t = time.time()
        for _index in range(100):
            percent_score = benchmark(GOLDEN_EN_RULES, tokenize_func)

        time_taken = time.time() - t

        print(f"{tokenize_func.__name__:30}{percent_score:0.2f}{time_taken * 1000 / 100:>10.2f}")
