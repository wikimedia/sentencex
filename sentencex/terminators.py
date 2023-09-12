# unicode code points with the \p{Sentence_Break=STerm} or \p{Sentence_Break=ATerm} properties that
# also have the \p{Terminal_Punctuation} property generated with Unicode::Tussle perl script and
# additional fullstops in unicode character sets : https://www.fileformat.info/info/unicode/char/search.htm?q=.&
# preview=entity
GLOBAL_SENTENCE_TERMINATORS = [
    "...",  # Horizontal Ellipsis
    "!",  # Exclamation Mark
    ".",  # Full Stop
    "?",  # Question Mark
    "։",  # Armenian Full Stop
    "؞",  # Arabic Sign Sallallahou Alayhe Wasallam
    "؟",  # Arabic Question Mark
    "۔",  # Arabic Full Stop
    "܀",  # Syriac End of Paragraph
    "܁",  # Syriac Supralinear Colon
    "܂",  # Syriac Sublinear Colon
    "߹",  # Nko Symbol Doorye
    "࠷",  # Samarkan Letter Do
    "࠹",  # Samarkan Letter Jho
    "࠽",  # Samarkan Letter Ro
    "࠾",  # Samarkan Letter Lo
    "।",  # Devanagari Danda
    "॥",  # Devanagari Double Danda
    "၊",  # Myanmar Sign Myanmar Phrase Stop
    "။",  # Myanmar Sign Myanmar Paragraph
    "።",  # Ethiopic Full Stop
    "፧",  # Ethiopic Colon
    "፨",  # Ethiopic Preface Colon
    "᙮",  # Ethiopic Question Mark
    "᜵",  # Buginese Vowel Sign E
    "᜶",  # Buginese Vowel Sign O
    "᠃",  # Mongolian Full Stop
    "᠉",  # Mongolian Birga
    "᥄",  # Buhid Virama
    "᥅",  # Buhid Punctuation Mark
    "᪨",  # Tai Tham Consonant Sign Medial Ra
    "᪩",  # Tai Tham Consonant Sign Medial La
    "᪪",  # Tai Tham Consonant Sign La Taa
    "᪫",  # Tai Tham Sign Mai Sak
    "᭚",  # Balinese Pameneng
    "᭛",  # Balinese Musical Symbol Combining Jublag
    "᭞",  # Sundanese Padasan Agung
    "᭟",  # Sundanese Paneken
    "᰻",  # Buhid Pamudpod
    "᰼",  # Buhid Pamudpod Han
    "᱾",  # Limbu Question Mark
    "᱿",  # Limbu Exclamation Mark
    "‼",  # Double Exclamation Mark
    "‽",  # Interrobang
    "⁇",  # Double Question Mark
    "⁈",  # Question Exclamation Mark
    "⁉",  # Exclamation Question Mark
    "⸮",  # Reversed Question Mark
    "⸼",  # Armenian Parenthesis Right
    "꓿",  # Yi Punctuation Small Comma
    "꘎",  # Vai Comma
    "꘏",  # Vai Full Stop
    "꛳",  # Batak Apostrophe
    "꛷",  # Batak Pangolat
    "꡶",  # Lanna Punctation Phrase
    "꡷",  # Lanna Punctation Paragraph
    "꣎",  # Ol Chiki Punctuation Mucaad
    "꣏",  # Ol Chiki Punctuation Double
    "꤯",  # Chakma Sign Visarga
    "꧈",  # Balinese Musical Symbol Left-Hand Open Dug
    "꧉",  # Balinese Musical Symbol Right-Hand Open Dug
    "꩝",  # Cham Consonant Sign Final H
    "꩞",  # Cham Consonant Sign Glottal Stop
    "꩟",  # Cham Consonant Sign M
    "꫰",  # Tai Viet Mai Khit
    "꫱",  # Tai Viet Vowel Ia
    "꯫",  # Meetei Mayek Cheikhei
    "﹒",  # Small Full Stop
    "﹖",  # Small Question Mark
    "﹗",  # Small Exclamation Mark
    "！",  # Fullwidth Exclamation Mark
    "．",  # Fullwidth Full Stop
    "？",  # Fullwidth Question Mark
    "ཕ",  # Tibetan Letter Pha
    "བ",  # Tibetan Letter Ba
    "བྷ",  # Tibetan Letter Bha
    "མ",  # Tibetan Letter Ma
    "ཙ",  # Tibetan Letter Tsa
    "၇",  # Myanmar Digit Seven
    "၈",  # Myanmar Digit Eight
    "Ⴞ",  # Georgian Letter Har
    "Ⴟ",  # Georgian Letter Hae
    "Ⴠ",  # Georgian Letter Hoe
    "Ⴡ",  # Georgian Letter Yu
    "ᅁ",  # Hangul Letter Yeorin Hieuh
    "ᅂ",  # Hangul Letter Yeorin Simeum
    "ᅃ",  # Hangul Letter Yeorin Cieuc
    "ᇅ",  # Hangul Letter Phieuph-Pieup
    "ᇆ",  # Hangul Letter Kapyeounphieuph
    "ᇍ",  # Hangul Letter Kapyeounhieuh
    "ᇞ",  # Hangul Letter Yang-Hieuh
    "ᇟ",  # Hangul Letter Yo-Yae
    "ሸ",  # Ethiopic Syllable Shee
    "ሹ",  # Ethiopic Syllable Shuu
    "ሻ",  # Ethiopic Syllable Shaa
    "ሼ",  # Ethiopic Syllable She
    "ኩ",  # Ethiopic Syllable Ku
    "ᑋ",  # Canadian Syllabics We
    "ᑌ",  # Canadian Syllabics West-Cree Pa
    "ᗂ",  # Canadian Syllabics South Slavey Lo
    "ᗃ",  # Canadian Syllabics South Slavey Lu
    "ᗉ",  # Canadian Syllabics Carrier Syllabic Yay
    "ᗊ",  # Canadian Syllabics Carrier Syllabic Yaa
    "ᗋ",  # Canadian Syllabics Carrier Syllabic Ywe
    "ᗌ",  # Canadian Syllabics Carrier Syllabic Ywi
    "ᗍ",  # Canadian Syllabics Carrier Syllabic Ywii
    "ᗎ",  # Canadian Syllabics Carrier Syllabic Ywo
    "ᗏ",  # Canadian Syllabics Carrier Syllabic Ywoo
    "ᗐ",  # Canadian Syllabics Carrier Syllabic Ywi
    "ᗗ",  # Canadian Syllabics Cree-Cha
    "ᙁ",  # Canadian Syllabics Slavey She
    "ᙂ",  # Canadian Syllabics Chipewyan Ga
    "᥄",  # Ethiopic Syllable Gwa
    "᥆",  # Ethiopic Syllable Gwo
    "ᩂ",  # Tai Tham Consonant Sign Low Ha
    "ᩃ",  # Tai Tham Consonant Sign High Ha
    "᱁",  # Ethiopic Syllable Hoa
    "᱂",  # Ethiopic Syllable Hoa
    "ỷ",  # Latin Small Letter Y With Tilde
    "Ỹ",  # Latin Capital Letter Y With Tilde
    "橮",  # CJK Unified Ideograph-6AEE
    "橯",  # CJK Unified Ideograph-6AEF
    "櫵",  # CJK Unified Ideograph-6AF5
    "欷",  # CJK Unified Ideograph-6B37
    "欸",  # CJK Unified Ideograph-6B38
    "歄",  # CJK Unified Ideograph-6B84
    "溘",  # CJK Unified Ideograph-6E98
    "벟",  # Hangul Syllable Eq
    "⳹",  # Greek Small Letter Ous
    "⳾",  # Greek Small Letter Psi
    "。",  # Ideographic Full Stop
    "︒",  # Presentation Form For Vertical Ideographic Full Stop
    "｡",  # Halfwidth Katakana Middle Dot
    "𖫵",  # Mongolian Vowel Separator
    "𖺘",  # Mongolian Letter Ali Gali U
    "𛲟",  # Hanifi Rohingya Sign Harbahay
    "𝪈",  # Mathematical Bold Capital U
]  # 150 symbols
