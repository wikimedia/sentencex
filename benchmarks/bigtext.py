import time

import blingfire
import nltk
import pysbd
import spacy
import stanza
import syntok.segmenter as syntok_segmenter
from syntok.tokenizer import Tokenizer

import sentencesegmenter

pysbd_segmenter = pysbd.Segmenter(language="en", clean=False, char_span=False)

nlp = spacy.blank("en")
nlp.add_pipe("sentencizer")
nlp_dep = spacy.load("en_core_web_sm", disable=["ner"])
stanza.download('en')
stanza_nlp = stanza.Pipeline(lang="en", processors="tokenize")

syntok_tokenizer = Tokenizer()


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

def sentencesegmenter_segment(text):
    return sentencesegmenter.segment("en", text)


if __name__ == "__main__":
    libraries = (
        blingfire_tokenize,
        nltk_tokenize,
        pysbd_tokenize,
        spacy_tokenize,
        spacy_dep_tokenize,
        stanza_tokenize,
        syntok_tokenize,
        sentencesegmenter_segment,
    )

    test_file="benchmarks/1661-0.txt"
    with open(test_file) as bigfile:
        big_text = bigfile.read()
        print("Library\tSpeed\tSentences")
        for tokenize_func in libraries:
            start = time.time()
            sentences = tokenize_func(big_text)
            time_taken = time.time() - start
            # print()
            # print(tokenize_func.__name__)
            # print("Speed : {:>20.2f} ms".format(time_taken * 1000))
            print(f"{tokenize_func.__name__}\t{time_taken * 1000}s\t{len(sentences)}")
            sentencesfile=open(f"benchmarks/{tokenize_func.__name__}.txt", "w")
            snum = 0
            for sentence in sentences:
                sentencesfile.write(f"{snum}:{sentence}\n")
                snum += 1
            sentencesfile.close()


