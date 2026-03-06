from sentencex import segment, get_sentence_boundaries
import time


def main():
    language_code = "en"
    text = "Hello world. This is a test."
    sentences = segment(language_code, text)
    print("Segmented sentences:", sentences)
    boundaries = get_sentence_boundaries(
        "en", "This is first sentence. This is another one."
    )
    print(boundaries)
    with open("../../benchmarks/fixtures/shakespeare.txt") as bigfile:
        text = bigfile.read()
    t = time.time()
    sentences = segment(language_code, text)
    time_taken = time.time() - t
    print("Speed : {:>20.2f} ms".format(time_taken * 1000))
    print(f"Sentences: {len(sentences)}")


if __name__ == "__main__":
    main()
