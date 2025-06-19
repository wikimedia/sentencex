from sentencex import segment

def main():
    language_code = "en"
    text = "Hello world. This is a test."
    sentences = segment(language_code, text)

    print("Segmented sentences:", sentences)

if __name__ == "__main__":
    main()
