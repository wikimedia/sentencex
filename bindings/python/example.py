from sentencex import segment, get_sentence_boundaries


def main():
    language_code = "en"
    text = "Hello world. This is a test."
    sentences = segment(language_code, text)
    print("Segmented sentences:", sentences)
    boundaries = get_sentence_boundaries(
        "en", "This is first sentence. This is another one."
    )
    print(boundaries)


if __name__ == "__main__":
    main()
