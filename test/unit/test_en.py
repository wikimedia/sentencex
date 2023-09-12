import pytest

from sentencex import segment

# ruff: noqa: E501
tests = [
    ("This is Dr. Watson", ["This is Dr. Watson"]),
    ("Roses Are Red. Violets Are Blue", ["Roses Are Red.", "Violets Are Blue"]),
    ("Hello! How are you?", ["Hello!", "How are you?"]),
    ("This is a test.", ["This is a test."]),
    ("Mr. Smith went to Washington.", ["Mr. Smith went to Washington."]),
    ("What a suprise?!", ["What a suprise?!"]),
    ("That's all folks...", ["That's all folks..."]),
    ("First line\nSecond line", ["First line\nSecond line"]),
    (
        "First line\nSecond line\n\nThird line",
        [
            "First line\nSecond line",
            "\n\n",
            "Third line",
        ],
    ),
    ("This is UK. Not US", ["This is UK.", "Not US"]),
    ("This balloon costs $1.20", ["This balloon costs $1.20"]),
    ("Hello World. My name is Jonas.", ["Hello World.", "My name is Jonas."]),
    ("What is your name? My name is Jonas.", ["What is your name?", "My name is Jonas."]),
    ("There it is! I found it.", ["There it is!", "I found it."]),
    ("My name is Jonas E. Smith.", ["My name is Jonas E. Smith."]),
    ("Please turn to p. 55.", ["Please turn to p. 55."]),
    ("Were Jane and co. at the party?", ["Were Jane and co. at the party?"]),
    (
        "They closed the deal with Pitt, Briggs & Co. at noon.",
        ["They closed the deal with Pitt, Briggs & Co. at noon."],
    ),
    pytest.param(
        "Let's ask Jane and co. They should know.",
        # Ideal result
        ["Let's ask Jane and co.", "They should know."],
        # Acceptable:
        # ["Let's ask Jane and co. They should know."],
        marks=pytest.mark.xfail,
    ),
    pytest.param(
        "They closed the deal with Pitt, Briggs & Co. It closed yesterday.",
        # Ideal result
        [
            "They closed the deal with Pitt, Briggs & Co.",
            "It closed yesterday.",
        ],
        # Acceptable:
        # ["They closed the deal with Pitt, Briggs & Co. It closed yesterday."],
        marks=pytest.mark.xfail,
    ),
    ("I can see Mt. Fuji from here.", ["I can see Mt. Fuji from here."]),
    (
        "St. Michael's Church is on 5th st. near the light.",
        ["St. Michael's Church is on 5th st. near the light."],
    ),
    ("That is JFK Jr.'s book.", ["That is JFK Jr.'s book."]),
    ("I visited the U.S.A. last year.", ["I visited the U.S.A. last year."]),
    pytest.param(
        "I live in the E.U. How about you?",
        ["I live in the E.U.", "How about you?"],
        marks=pytest.mark.xfail,
    ),
    pytest.param(
        "I live in the U.S. How about you?",
        ["I live in the U.S.", "How about you?"],
        marks=pytest.mark.xfail,
    ),
    (
        "I work for the U.S. Government in Virginia.",
        ["I work for the U.S. Government in Virginia."],
    ),
    ("I have lived in the U.S. for 20 years.", ["I have lived in the U.S. for 20 years."]),
    # Most difficult sentence to crack
    pytest.param(
        "At 5 a.m. Mr. Smith went to the bank. \
            He left the bank at 6 P.M. Mr. Smith then went to the store.",
        # Ideal result:
        [
            "At 5 a.m. Mr. Smith went to the bank.",
            "He left the bank at 6 P.M.",
            "Mr. Smith then went to the store.",
        ],
        # Acceptable:
        # [
        #     "At 5 a.m. Mr. Smith went to the bank.",
        #     "He left the bank at 6 P.M. Mr. Smith then went to the store.",
        # ],
        marks=pytest.mark.xfail,
    ),
    ("She has $100.00 in her bag.", ["She has $100.00 in her bag."]),
    ("She has $100.00. It is in her bag.", ["She has $100.00.", "It is in her bag."]),
    (
        "He teaches science (He previously worked for 5 years as an engineer.) at the local University.",
        [
            "He teaches science (He previously worked for 5 years as an engineer.) at the local University."
        ],
    ),
    (
        "Her email is Jane.Doe@example.com. I sent her an email.",
        [
            "Her email is Jane.Doe@example.com.",
            "I sent her an email.",
        ],
    ),
    (
        "The site is, https,//www.example.50.com/new-site/awesome_content.html. Please check it out.",
        [
            "The site is, https,//www.example.50.com/new-site/awesome_content.html.",
            "Please check it out.",
        ],
    ),
    (
        "She turned to him, 'This is great.' she said.",
        ["She turned to him, 'This is great.' she said."],
    ),
    (
        'She turned to him, "This is great." she said.',
        ['She turned to him, "This is great." she said.'],
    ),
    pytest.param(
        'She turned to him, "This is great." She held the book out to show him.',
        [
            'She turned to him, "This is great."',
            "She held the book out to show him.",
        ],
        marks=pytest.mark.xfail,
    ),
    ("Hello!! Long time no see.", ["Hello!!", "Long time no see."]),
    ("Hello?? Who is there?", ["Hello??", "Who is there?"]),
    ("Hello!? Is that you?", ["Hello!?", "Is that you?"]),
    ("Hello?! Is that you?", ["Hello?!", "Is that you?"]),
    # Lists are not supported now
    # ("1.) The first item 2.) The second item", ["1.) The first item", "2.) The second item"]),
    # ("1.) The first item. 2.) The second item.", ["1.) The first item.", "2.) The second item."]),
    # ("1) The first item 2) The second item", ["1) The first item", "2) The second item"]),
    # ("1) The first item. 2) The second item.", ["1) The first item.", "2) The second item."]),
    # ("1. The first item 2. The second item", ["1. The first item", "2. The second item"]),
    # ("1. The first item. 2. The second item.", ["1. The first item.", "2. The second item."]),
    # (
    #     "• 9. The first item • 10. The second item",
    #     [
    #         "• 9. The first item",
    #         "• 10. The second item",
    #     ],
    # ),
    # ("⁃9. The first item ⁃10. The second item", ["⁃9. The first item", "⁃10. The second item"]),
    # (
    #     "a. The first item b. The second item c. The third list item",
    #     [
    #         "a. The first item",
    #         "b. The second item",
    #         "c. The third list item",
    #     ],
    # ),
    (
        "You can find it at N°. 1026.253.553. That is where the treasure is.",
        [
            "You can find it at N°. 1026.253.553.",
            "That is where the treasure is.",
        ],
    ),
    (
        "She works at Yahoo! in the accounting department.",
        ["She works at Yahoo! in the accounting department."],
    ),
    pytest.param(
        "We make a good team, you and I. Did you see Albert I. Jones yesterday?",
        [
            "We make a good team, you and I.",
            "Did you see Albert I. Jones yesterday?",
        ],
        marks=pytest.mark.xfail,
    ),
    (
        r"Thoreau argues that by simplifying one’s life, “the laws of the universe will appear less complex. . . .”",
        [
            "Thoreau argues that by simplifying one’s life, “the laws of the universe will appear less complex. . . .”"
        ],
    ),
    (
        """"Bohr [...] used the analogy of parallel stairways [...]" (Smith 55).""",
        ['"Bohr [...] used the analogy of parallel stairways [...]" (Smith 55).'],
    ),
    pytest.param(
        "If words are left off at the end of a sentence, and that is all that is omitted, indicate the omission with ellipsis marks (preceded and followed by a space) and then indicate the end of the sentence with a period . . . . Next sentence.",
        [
            "If words are left off at the end of a sentence, and that is all that is omitted, indicate the omission with ellipsis marks (preceded and followed by a space) and then indicate the end of the sentence with a period . . . .",
            "Next sentence.",
        ],
        marks=pytest.mark.xfail,
    ),
    (
        "I never meant that.... She left the store.",
        [
            "I never meant that....",
            "She left the store.",
        ],
    ),
    pytest.param(
        "I wasn’t really ... well, what I mean...see . . . what I'm saying, the thing is . . . I didn’t mean it.",
        [
            "I wasn’t really ... well, what I mean...see . . . what I'm saying, the thing is . . . I didn’t mean it."
        ],
        marks=pytest.mark.xfail,
    ),
    pytest.param(
        "One further habit which was somewhat weakened . . . was that of combining words into self-interpreting compounds. . . . The practice was not abandoned. . . .",
        [
            "One further habit which was somewhat weakened . . . was that of combining words into self-interpreting compounds.",
            ". . . The practice was not abandoned. . . .",
        ],
        marks=pytest.mark.xfail,
    ),
    (
        "Saint Maximus (died 250) is a Christian saint and martyr.[1] The emperor Decius published a decree ordering the veneration of busts of the deified emperors.",
        [
            "Saint Maximus (died 250) is a Christian saint and martyr.[1]",
            "The emperor Decius published a decree ordering the veneration of busts of the deified emperors.",
        ],
    ),
    (
        "Differing agendas can potentially create an understanding gap in a consultation.11 12 Take the example of one of the most common presentations in ill health: the common cold.",
        [
            "Differing agendas can potentially create an understanding gap in a consultation.11 12 Take the example of one of the most common presentations in ill health: the common cold."
        ],
    ),
    (
        "Its traditional use[1] is well documented in the ethnobotanical literature [2–11]. Leaves, buds, tar and essential oils are used to treat a wide spectrum of diseases.",
        [
            "Its traditional use[1] is well documented in the ethnobotanical literature [2–11].",
            "Leaves, buds, tar and essential oils are used to treat a wide spectrum of diseases.",
        ],
    ),
    (
        "Thus increasing the desire for political reform both in Lancashire and in the country at large.[7][8] This was a serious misdemeanour,[16] encouraging them to declare the assembly illegal as soon as it was announced on 31 July.[17][18] The radicals sought a second opinion on the meeting's legality.",
        [
            "Thus increasing the desire for political reform both in Lancashire and in the country at large.[7][8]",
            "This was a serious misdemeanour,[16] encouraging them to declare the assembly illegal as soon as it was announced on 31 July.[17][18]",
            "The radicals sought a second opinion on the meeting's legality.",
        ],
    ),
    (
        "“Why, indeed?” murmured Holmes. “Your Majesty had not spoken before I \nwas aware that I was addressing Wilhelm Gottsreich Sigismond von \nOrmstein, Grand Duke of Cassel-Felstein, and hereditary King of \nBohemia.”",
        [
            "“Why, indeed?” murmured Holmes.",
            "“Your Majesty had not spoken before I \nwas aware that I was addressing Wilhelm Gottsreich Sigismond von \nOrmstein, Grand Duke of Cassel-Felstein, and hereditary King of \nBohemia.”",
        ],
    ),
    (
        "“How many? I don’t know.”",
        [
            "“How many? I don’t know.”",
        ],
    ),
]


@pytest.mark.parametrize("text, expected_sents", tests)
def test_segment(text, expected_sents):
    assert list(segment("en", text)) == expected_sents
