use regex::Regex;
use std::collections::HashMap;

pub const ROMAN_NUMERALS: [&str; 20] = [
    "i", "ii", "iii", "iv", "v", "vi", "vii", "viii", "ix", "x", "xi", "xii", "xiii", "xiv", "xv",
    "xvi", "xvii", "xviii", "xix", "xx",
];

pub fn get_quote_pairs() -> HashMap<&'static str, &'static str> {
    let mut quote_pairs = HashMap::new();
    quote_pairs.insert("\"", "\"");
    quote_pairs.insert(" '", "'"); // Need a space before ' to avoid capturing don't, l'Avv, etc.
    quote_pairs.insert("«", "»");
    quote_pairs.insert("‘", "’");
    quote_pairs.insert("‚", "‚");
    quote_pairs.insert("“", "”");
    quote_pairs.insert("‛", "‛");
    quote_pairs.insert("„", "“");
    quote_pairs.insert("‟", "‟");
    quote_pairs.insert("‹", "›");
    quote_pairs.insert("《", "》");
    quote_pairs.insert("「", "」");
    quote_pairs
}

use std::sync::LazyLock;

pub static PARENS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\(（<{\[](?:[^\)\]}>）]|\\[\)\]}>）])*[\)\]}>）]").unwrap());

pub static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,7}").unwrap());

pub static NUMBERED_REFERENCE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\s*\[\d+])+").unwrap());

pub static SPACE_AFTER_SEPARATOR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+").unwrap());

pub static QUOTES_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let quote_pairs = get_quote_pairs();
    let patterns: Vec<String> = quote_pairs
        .iter()
        .map(|(left, right)| format!(r"{}(\n|.)*?{}", regex::escape(left), regex::escape(right)))
        .collect();
    let quotes_regex_str = patterns.join("|");
    Regex::new(&quotes_regex_str).unwrap()
});

pub const EXCLAMATION_WORDS: [&str; 17] = [
    "!Xũ",
    "!Kung",
    "ǃʼOǃKung",
    "!Xuun",
    "!Kung-Ekoka",
    "ǃHu",
    "ǃKhung",
    "ǃKu",
    "ǃung",
    "ǃXo",
    "ǃXû",
    "ǃXung",
    "ǃXũ",
    "!Xun",
    "Yahoo!",
    "Y!J",
    "Yum!",
];

// unicode code points generated with Unicode::Tussle perl script:
// unichars -aBbs '[\p{Sentence_Break=STerm}\p{Sentence_Break=ATerm}]' | awk '$2="\""$2"\", //"'
// Refer: https://www.unicode.org/Public/UCD/latest/ucd/auxiliary/SentenceBreakProperty.txt
pub const GLOBAL_SENTENCE_TERMINATORS: [&str; 155] = [
    "!",  // U+00021 BC=ON BLK=Basic_Latin SC=Common EXCLAMATION MARK
    ".",  // U+0002E BC=CS BLK=Basic_Latin SC=Common FULL STOP
    "?",  // U+0003F BC=ON BLK=Basic_Latin SC=Common QUESTION MARK
    "։",  // U+00589 BC=L BLK=Armenian SC=Armenian ARMENIAN FULL STOP
    "؝",  // U+0061D BC=AL BLK=Arabic SC=Arabic ARABIC END OF TEXT MARK
    "؞",  // U+0061E BC=AL BLK=Arabic SC=Arabic ARABIC TRIPLE DOT PUNCTUATION MARK
    "؟",  // U+0061F BC=AL BLK=Arabic SC=Common ARABIC QUESTION MARK
    "۔",  // U+006D4 BC=AL BLK=Arabic SC=Arabic ARABIC FULL STOP
    "܀",  // U+00700 BC=AL BLK=Syriac SC=Syriac SYRIAC END OF PARAGRAPH
    "܁",  // U+00701 BC=AL BLK=Syriac SC=Syriac SYRIAC SUPRALINEAR FULL STOP
    "܂",  // U+00702 BC=AL BLK=Syriac SC=Syriac SYRIAC SUBLINEAR FULL STOP
    "߹",  // U+007F9 BC=ON BLK=NKo SC=Nko NKO EXCLAMATION MARK
    "࠷",  // U+00837 BC=R BLK=Samaritan SC=Samaritan SAMARITAN PUNCTUATION MELODIC QITSA
    "࠹",  // U+00839 BC=R BLK=Samaritan SC=Samaritan SAMARITAN PUNCTUATION QITSA
    "࠽",  // U+0083D BC=R BLK=Samaritan SC=Samaritan SAMARITAN PUNCTUATION SOF MASHFAAT
    "࠾",  // U+0083E BC=R BLK=Samaritan SC=Samaritan SAMARITAN PUNCTUATION ANNAAU
    "।",  // U+00964 BC=L BLK=Devanagari SC=Common DEVANAGARI DANDA
    "॥",  // U+00965 BC=L BLK=Devanagari SC=Common DEVANAGARI DOUBLE DANDA
    "၊",  // U+0104A BC=L BLK=Myanmar SC=Myanmar MYANMAR SIGN LITTLE SECTION
    "။",  // U+0104B BC=L BLK=Myanmar SC=Myanmar MYANMAR SIGN SECTION
    "።",  // U+01362 BC=L BLK=Ethiopic SC=Ethiopic ETHIOPIC FULL STOP
    "፧",  // U+01367 BC=L BLK=Ethiopic SC=Ethiopic ETHIOPIC QUESTION MARK
    "፨",  // U+01368 BC=L BLK=Ethiopic SC=Ethiopic ETHIOPIC PARAGRAPH SEPARATOR
    "᙮", // U+0166E BC=L BLK=Unified_Canadian_Aboriginal_Syllabics SC=Canadian_Aboriginal CANADIAN SYLLABICS FULL STOP
    "᜵", // U+01735 BC=L BLK=Hanunoo SC=Common PHILIPPINE SINGLE PUNCTUATION
    "᜶", // U+01736 BC=L BLK=Hanunoo SC=Common PHILIPPINE DOUBLE PUNCTUATION
    "᠃", // U+01803 BC=ON BLK=Mongolian SC=Common MONGOLIAN FULL STOP
    "᠉", // U+01809 BC=ON BLK=Mongolian SC=Mongolian MONGOLIAN MANCHU FULL STOP
    "᥄", // U+01944 BC=ON BLK=Limbu SC=Limbu LIMBU EXCLAMATION MARK
    "᥅", // U+01945 BC=ON BLK=Limbu SC=Limbu LIMBU QUESTION MARK
    "᪨", // U+01AA8 BC=L BLK=Tai_Tham SC=Tai_Tham TAI THAM SIGN KAAN
    "᪩", // U+01AA9 BC=L BLK=Tai_Tham SC=Tai_Tham TAI THAM SIGN KAANKUU
    "᪪", // U+01AAA BC=L BLK=Tai_Tham SC=Tai_Tham TAI THAM SIGN SATKAAN
    "᪫", // U+01AAB BC=L BLK=Tai_Tham SC=Tai_Tham TAI THAM SIGN SATKAANKUU
    "᭚", // U+01B5A BC=L BLK=Balinese SC=Balinese BALINESE PANTI
    "᭛", // U+01B5B BC=L BLK=Balinese SC=Balinese BALINESE PAMADA
    "᭞", // U+01B5E BC=L BLK=Balinese SC=Balinese BALINESE CARIK SIKI
    "᭟", // U+01B5F BC=L BLK=Balinese SC=Balinese BALINESE CARIK PAREREN
    "᭽", // U+01B7D BC=L BLK=Balinese SC=Balinese BALINESE PANTI LANTANG
    "᭾", // U+01B7E BC=L BLK=Balinese SC=Balinese BALINESE PAMADA LANTANG
    "᰻", // U+01C3B BC=L BLK=Lepcha SC=Lepcha LEPCHA PUNCTUATION TA-ROL
    "᰼", // U+01C3C BC=L BLK=Lepcha SC=Lepcha LEPCHA PUNCTUATION NYET THYOOM TA-ROL
    "᱾", // U+01C7E BC=L BLK=Ol_Chiki SC=Ol_Chiki OL CHIKI PUNCTUATION MUCAAD
    "᱿", // U+01C7F BC=L BLK=Ol_Chiki SC=Ol_Chiki OL CHIKI PUNCTUATION DOUBLE MUCAAD
    "․", // U+02024 BC=ON BLK=General_Punctuation SC=Common ONE DOT LEADER
    "‼", // U+0203C BC=ON BLK=General_Punctuation SC=Common DOUBLE EXCLAMATION MARK
    "‽", // U+0203D BC=ON BLK=General_Punctuation SC=Common INTERROBANG
    "⁇", // U+02047 BC=ON BLK=General_Punctuation SC=Common DOUBLE QUESTION MARK
    "⁈", // U+02048 BC=ON BLK=General_Punctuation SC=Common QUESTION EXCLAMATION MARK
    "⁉", // U+02049 BC=ON BLK=General_Punctuation SC=Common EXCLAMATION QUESTION MARK
    "⸮", // U+02E2E BC=ON BLK=Supplemental_Punctuation SC=Common REVERSED QUESTION MARK
    "⸼", // U+02E3C BC=ON BLK=Supplemental_Punctuation SC=Common STENOGRAPHIC FULL STOP
    "⹓", // U+02E53 BC=ON BLK=Supplemental_Punctuation SC=Common MEDIEVAL EXCLAMATION MARK
    "⹔", // U+02E54 BC=ON BLK=Supplemental_Punctuation SC=Common MEDIEVAL QUESTION MARK
    "꓿", // U+0A4FF BC=L BLK=Lisu SC=Lisu LISU PUNCTUATION FULL STOP
    "꘎", // U+0A60E BC=ON BLK=Vai SC=Vai VAI FULL STOP
    "꘏", // U+0A60F BC=ON BLK=Vai SC=Vai VAI QUESTION MARK
    "꛳", // U+0A6F3 BC=L BLK=Bamum SC=Bamum BAMUM FULL STOP
    "꛷", // U+0A6F7 BC=L BLK=Bamum SC=Bamum BAMUM QUESTION MARK
    "꡶", // U+0A876 BC=ON BLK=Phags-pa SC=Phags_Pa PHAGS-PA MARK SHAD
    "꡷", // U+0A877 BC=ON BLK=Phags-pa SC=Phags_Pa PHAGS-PA MARK DOUBLE SHAD
    "꣎", // U+0A8CE BC=L BLK=Saurashtra SC=Saurashtra SAURASHTRA DANDA
    "꣏", // U+0A8CF BC=L BLK=Saurashtra SC=Saurashtra SAURASHTRA DOUBLE DANDA
    "꤯", // U+0A92F BC=L BLK=Kayah_Li SC=Kayah_Li KAYAH LI SIGN SHYA
    "꧈", // U+0A9C8 BC=L BLK=Javanese SC=Javanese JAVANESE PADA LINGSA
    "꧉", // U+0A9C9 BC=L BLK=Javanese SC=Javanese JAVANESE PADA LUNGSI
    "꩝", // U+0AA5D BC=L BLK=Cham SC=Cham CHAM PUNCTUATION DANDA
    "꩞", // U+0AA5E BC=L BLK=Cham SC=Cham CHAM PUNCTUATION DOUBLE DANDA
    "꩟", // U+0AA5F BC=L BLK=Cham SC=Cham CHAM PUNCTUATION TRIPLE DANDA
    "꫰", // U+0AAF0 BC=L BLK=Meetei_Mayek_Extensions SC=Meetei_Mayek MEETEI MAYEK CHEIKHAN
    "꫱", // U+0AAF1 BC=L BLK=Meetei_Mayek_Extensions SC=Meetei_Mayek MEETEI MAYEK AHANG KHUDAM
    "꯫", // U+0ABEB BC=L BLK=Meetei_Mayek SC=Meetei_Mayek MEETEI MAYEK CHEIKHEI
    "﹒", // U+0FE52 BC=CS BLK=Small_Form_Variants SC=Common SMALL FULL STOP
    "﹖", // U+0FE56 BC=ON BLK=Small_Form_Variants SC=Common SMALL QUESTION MARK
    "﹗", // U+0FE57 BC=ON BLK=Small_Form_Variants SC=Common SMALL EXCLAMATION MARK
    "！", // U+0FF01 BC=ON BLK=Halfwidth_and_Fullwidth_Forms SC=Common FULLWIDTH EXCLAMATION MARK
    "．", // U+0FF0E BC=CS BLK=Halfwidth_and_Fullwidth_Forms SC=Common FULLWIDTH FULL STOP
    "？", // U+0FF1F BC=ON BLK=Halfwidth_and_Fullwidth_Forms SC=Common FULLWIDTH QUESTION MARK
    "𐩖", // U+10A56 BC=R BLK=Kharoshthi SC=Kharoshthi KHAROSHTHI PUNCTUATION DANDA
    "𐩗", // U+10A57 BC=R BLK=Kharoshthi SC=Kharoshthi KHAROSHTHI PUNCTUATION DOUBLE DANDA
    "𐽕", // U+10F55 BC=AL BLK=Sogdian SC=Sogdian SOGDIAN PUNCTUATION TWO VERTICAL BARS
    "𐽖", // U+10F56 BC=AL BLK=Sogdian SC=Sogdian SOGDIAN PUNCTUATION TWO VERTICAL BARS WITH DOTS
    "𐽗", // U+10F57 BC=AL BLK=Sogdian SC=Sogdian SOGDIAN PUNCTUATION CIRCLE WITH DOT
    "𐽘", // U+10F58 BC=AL BLK=Sogdian SC=Sogdian SOGDIAN PUNCTUATION TWO CIRCLES WITH DOTS
    "𐽙", // U+10F59 BC=AL BLK=Sogdian SC=Sogdian SOGDIAN PUNCTUATION HALF CIRCLE WITH DOT
    "𐾆", // U+10F86 BC=R BLK=Old_Uyghur SC=Old_Uyghur OLD UYGHUR PUNCTUATION BAR
    "𐾇", // U+10F87 BC=R BLK=Old_Uyghur SC=Old_Uyghur OLD UYGHUR PUNCTUATION TWO BARS
    "𐾈", // U+10F88 BC=R BLK=Old_Uyghur SC=Old_Uyghur OLD UYGHUR PUNCTUATION TWO DOTS
    "𐾉", // U+10F89 BC=R BLK=Old_Uyghur SC=Old_Uyghur OLD UYGHUR PUNCTUATION FOUR DOTS
    "𑁇", // U+11047 BC=L BLK=Brahmi SC=Brahmi BRAHMI DANDA
    "𑁈", // U+11048 BC=L BLK=Brahmi SC=Brahmi BRAHMI DOUBLE DANDA
    "𑂾", // U+110BE BC=L BLK=Kaithi SC=Kaithi KAITHI SECTION MARK
    "𑂿", // U+110BF BC=L BLK=Kaithi SC=Kaithi KAITHI DOUBLE SECTION MARK
    "𑃀", // U+110C0 BC=L BLK=Kaithi SC=Kaithi KAITHI DANDA
    "𑃁", // U+110C1 BC=L BLK=Kaithi SC=Kaithi KAITHI DOUBLE DANDA
    "𑅁", // U+11141 BC=L BLK=Chakma SC=Chakma CHAKMA DANDA
    "𑅂", // U+11142 BC=L BLK=Chakma SC=Chakma CHAKMA DOUBLE DANDA
    "𑅃", // U+11143 BC=L BLK=Chakma SC=Chakma CHAKMA QUESTION MARK
    "𑇅", // U+111C5 BC=L BLK=Sharada SC=Sharada SHARADA DANDA
    "𑇆", // U+111C6 BC=L BLK=Sharada SC=Sharada SHARADA DOUBLE DANDA
    "𑇍", // U+111CD BC=L BLK=Sharada SC=Sharada SHARADA SUTRA MARK
    "𑇞", // U+111DE BC=L BLK=Sharada SC=Sharada SHARADA SECTION MARK-1
    "𑇟", // U+111DF BC=L BLK=Sharada SC=Sharada SHARADA SECTION MARK-2
    "𑈸", // U+11238 BC=L BLK=Khojki SC=Khojki KHOJKI DANDA
    "𑈹", // U+11239 BC=L BLK=Khojki SC=Khojki KHOJKI DOUBLE DANDA
    "𑈻", // U+1123B BC=L BLK=Khojki SC=Khojki KHOJKI SECTION MARK
    "𑈼", // U+1123C BC=L BLK=Khojki SC=Khojki KHOJKI DOUBLE SECTION MARK
    "𑊩", // U+112A9 BC=L BLK=Multani SC=Multani MULTANI SECTION MARK
    "𑑋", // U+1144B BC=L BLK=Newa SC=Newa NEWA DANDA
    "𑑌", // U+1144C BC=L BLK=Newa SC=Newa NEWA DOUBLE DANDA
    "𑗂", // U+115C2 BC=L BLK=Siddham SC=Siddham SIDDHAM DANDA
    "𑗃", // U+115C3 BC=L BLK=Siddham SC=Siddham SIDDHAM DOUBLE DANDA
    "𑗉", // U+115C9 BC=L BLK=Siddham SC=Siddham SIDDHAM END OF TEXT MARK
    "𑗊", // U+115CA BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH TRIDENT AND U-SHAPED ORNAMENTS
    "𑗋", // U+115CB BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH TRIDENT AND DOTTED CRESCENTS
    "𑗌", // U+115CC BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH RAYS AND DOTTED CRESCENTS
    "𑗍", // U+115CD BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH RAYS AND DOTTED DOUBLE CRESCENTS
    "𑗎", // U+115CE BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH RAYS AND DOTTED TRIPLE CRESCENTS
    "𑗏", // U+115CF BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK DOUBLE RING
    "𑗐", // U+115D0 BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK DOUBLE RING WITH RAYS
    "𑗑", // U+115D1 BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH DOUBLE CRESCENTS
    "𑗒", // U+115D2 BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH TRIPLE CRESCENTS
    "𑗓", // U+115D3 BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH QUADRUPLE CRESCENTS
    "𑗔", // U+115D4 BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH SEPTUPLE CRESCENTS
    "𑗕", // U+115D5 BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH CIRCLES AND RAYS
    "𑗖", // U+115D6 BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH CIRCLES AND TWO ENCLOSURES
    "𑗗", // U+115D7 BC=L BLK=Siddham SC=Siddham SIDDHAM SECTION MARK WITH CIRCLES AND FOUR ENCLOSURES
    "𑙁", // U+11641 BC=L BLK=Modi SC=Modi MODI DANDA
    "𑙂", // U+11642 BC=L BLK=Modi SC=Modi MODI DOUBLE DANDA
    "𑜼", // U+1173C BC=L BLK=Ahom SC=Ahom AHOM SIGN SMALL SECTION
    "𑜽", // U+1173D BC=L BLK=Ahom SC=Ahom AHOM SIGN SECTION
    "𑜾", // U+1173E BC=L BLK=Ahom SC=Ahom AHOM SIGN RULAI
    "𑥄", // U+11944 BC=L BLK=Dives_Akuru SC=Dives_Akuru DIVES AKURU DOUBLE DANDA
    "𑥆", // U+11946 BC=L BLK=Dives_Akuru SC=Dives_Akuru DIVES AKURU END OF TEXT MARK
    "𑩂", // U+11A42 BC=L BLK=Zanabazar_Square SC=Zanabazar_Square ZANABAZAR SQUARE MARK SHAD
    "𑩃", // U+11A43 BC=L BLK=Zanabazar_Square SC=Zanabazar_Square ZANABAZAR SQUARE MARK DOUBLE SHAD
    "𑪛", // U+11A9B BC=L BLK=Soyombo SC=Soyombo SOYOMBO MARK SHAD
    "𑪜", // U+11A9C BC=L BLK=Soyombo SC=Soyombo SOYOMBO MARK DOUBLE SHAD
    "𑱁", // U+11C41 BC=L BLK=Bhaiksuki SC=Bhaiksuki BHAIKSUKI DANDA
    "𑱂", // U+11C42 BC=L BLK=Bhaiksuki SC=Bhaiksuki BHAIKSUKI DOUBLE DANDA
    "𑻷", // U+11EF7 BC=L BLK=Makasar SC=Makasar MAKASAR PASSIMBANG
    "𑻸", // U+11EF8 BC=L BLK=Makasar SC=Makasar MAKASAR END OF SECTION
    "𑽃", // U+11F43 BC=L BLK=Kawi SC=Kawi KAWI DANDA
    "𑽄", // U+11F44 BC=L BLK=Kawi SC=Kawi KAWI DOUBLE DANDA
    "𖩮", // U+16A6E BC=L BLK=Mro SC=Mro MRO DANDA
    "𖩯", // U+16A6F BC=L BLK=Mro SC=Mro MRO DOUBLE DANDA
    "𖫵", // U+16AF5 BC=L BLK=Bassa_Vah SC=Bassa_Vah BASSA VAH FULL STOP
    "𖬷", // U+16B37 BC=L BLK=Pahawh_Hmong SC=Pahawh_Hmong PAHAWH HMONG SIGN VOS THOM
    "𖬸", // U+16B38 BC=L BLK=Pahawh_Hmong SC=Pahawh_Hmong PAHAWH HMONG SIGN VOS TSHAB CEEB
    "𖭄", // U+16B44 BC=L BLK=Pahawh_Hmong SC=Pahawh_Hmong PAHAWH HMONG SIGN XAUS
    "𖺘", // U+16E98 BC=L BLK=Medefaidrin SC=Medefaidrin MEDEFAIDRIN FULL STOP
    "𛲟", // U+1BC9F BC=L BLK=Duployan SC=Duployan DUPLOYAN PUNCTUATION CHINOOK FULL STOP
    "𝪈", // U+1DA88 BC=L BLK=Sutton_SignWriting SC=SignWriting SIGNWRITING FULL STOP
    //  Additional manual entries.
    "。", // U+3002 IDEOGRAPHIC FULL STOP
    "｡",  // U+FF61 HALFWIDTH IDEOGRAPHIC FULL STOP
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotes_regex_greek_basic() {
        let text = "Ο γιατρός είπε: «Η κατάσταση είναι σταθερή».";
        let matches: Vec<_> = QUOTES_REGEX
            .find_iter(text)
            .map(|m| &text[m.start()..m.end()])
            .collect();
        assert_eq!(matches, vec!["«Η κατάσταση είναι σταθερή»"]);
    }

    #[test]
    fn test_quotes_regex_greek_multiple() {
        let text = "«Καλημέρα» είπε. «Πώς είσαι;»";
        let matches: Vec<_> = QUOTES_REGEX
            .find_iter(text)
            .map(|m| &text[m.start()..m.end()])
            .collect();
        assert_eq!(matches, vec!["«Καλημέρα»", "«Πώς είσαι;»"]);
    }

    #[test]
    fn test_quotes_regex_greek_multiline() {
        let text = "Παράδειγμα:\n«Πρώτη γραμμή\nΔεύτερη γραμμή» τέλος.";
        let matches: Vec<_> = QUOTES_REGEX
            .find_iter(text)
            .map(|m| &text[m.start()..m.end()])
            .collect();
        assert_eq!(matches, vec!["«Πρώτη γραμμή\nΔεύτερη γραμμή»"]);
    }

    #[test]
    fn test_quotes_regex_other_double_quotes() {
        // Ensure standard ASCII quotes still work
        let text = r#"He said, "Hello" and left."#;
        let matches: Vec<_> = QUOTES_REGEX
            .find_iter(text)
            .map(|m| &text[m.start()..m.end()])
            .collect();
        assert_eq!(matches, vec!["\"Hello\""]);
    }

    #[test]
    fn test_quotes_regex_all_quote_types() {
        // Test all quote pairs defined in get_quote_pairs
        let test_cases = vec![
            ("Standard double: \"Hello\"", vec!["\"Hello\""]),
            ("Greek: «Γεια σου»", vec!["«Γεια σου»"]),
            ("Curved single: 'Hi'", vec![" 'Hi'"]),
            ("German-style: „Hallo“", vec!["„Hallo“"]),
            ("Single angular: ‹Bonjour›", vec!["‹Bonjour›"]),
            ("CJK: 「こんにちは」", vec!["「こんにちは」"]),
            ("Chinese: 《你好》", vec!["《你好》"]),
        ];

        for (text, expected) in test_cases {
            let matches: Vec<_> = QUOTES_REGEX
                .find_iter(text)
                .map(|m| &text[m.start()..m.end()])
                .collect();
            assert_eq!(matches, expected, "Failed for text: {}", text);
        }
    }

    #[test]
    fn test_quotes_regex_edge_cases() {
        // Empty quotes
        let text = "Empty: «»";
        let matches: Vec<_> = QUOTES_REGEX
            .find_iter(text)
            .map(|m| &text[m.start()..m.end()])
            .collect();
        assert_eq!(matches, vec!["«»"]);

        // Nested quotes (should match the outer ones)
        let text = "Nested: «Outer 'inner' quotes»";
        let matches: Vec<_> = QUOTES_REGEX
            .find_iter(text)
            .map(|m| &text[m.start()..m.end()])
            .collect();
        assert_eq!(matches, vec!["«Outer 'inner' quotes»"]);

        // No quotes
        let text = "No quotes here at all.";
        let matches: Vec<_> = QUOTES_REGEX
            .find_iter(text)
            .map(|m| &text[m.start()..m.end()])
            .collect();
        assert_eq!(matches, Vec::<&str>::new());
    }

    #[test]
    fn test_quotes_regex_with_punctuation() {
        // Quotes with various punctuation inside
        let test_cases = vec![
            ("Question: «Πώς είσαι;»", vec!["«Πώς είσαι;»"]),
            ("Exclamation: «Γεια σου!»", vec!["«Γεια σου!»"]),
            ("Period: «Καλημέρα.»", vec!["«Καλημέρα.»"]),
            (
                "Complex: «Ελα, πώς είσαι; Καλά!»",
                vec!["«Ελα, πώς είσαι; Καλά!»"],
            ),
        ];

        for (text, expected) in test_cases {
            let matches: Vec<_> = QUOTES_REGEX
                .find_iter(text)
                .map(|m| &text[m.start()..m.end()])
                .collect();
            assert_eq!(matches, expected, "Failed for text: {}", text);
        }
    }

    #[test]
    fn test_quotes_regex_debug_pattern() {
        // Let's see what the actual regex pattern looks like
        let quote_pairs = get_quote_pairs();
        let patterns: Vec<String> = quote_pairs
            .iter()
            .map(|(left, right)| {
                format!(r"{}(\n|.)*?{}", regex::escape(left), regex::escape(right))
            })
            .collect();
        let quotes_regex_str = patterns.join("|");

        println!("QUOTES_REGEX pattern: {}", quotes_regex_str);

        // Verify that Greek quotes are in the pattern
        assert!(quotes_regex_str.contains("«"));
        assert!(quotes_regex_str.contains("»"));
    }
}
