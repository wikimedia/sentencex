import time

import sentencesegmenter

if __name__ == "__main__":
    with open("benchmark/1661-0.txt") as bigfile:
        big_text = bigfile.read()
        start = time.time()
        sentences = sentencesegmenter.segment("en", big_text)
        time_taken = time.time() - start
        print("Speed : {:>10.2f} ms".format(time_taken * 1000))
        print(f"Sentences : {len(sentences)}")

    # with open("benchmark/1661-0.sentences.txt", "w") as sentencesfile:
    #     sindex=0
    #     for sentence in sentences:
    #         sentencesfile.write(f"{sindex}: {sentence}\n")
    #         sindex+=1
    #     sentencesfile.close()
