use std::sync::LazyLock;

use super::Language;

#[derive(Debug, Clone)]
pub struct Tamil {}
static TAMIL_ABBREVIATIONS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let vowel_signs = vec!["ா", "ி", "ீ", "ু", "ূ", "ে", "ে", "ৈ", "ও", "ো", "ৌ"];
    let vowels = vec!["அ", "ஆ", "இ", "ஈ", "உ", "ஊ", "எ", "ஏ", "ஐ", "ஒ", "ஓ", "ஔ"];
    let consonants = vec![
        "க", "ங", "ச", "ஞ", "ட", "ண", "த", "ந", "ப", "ம", "ய", "ர", "ல", "வ", "ழ", "ள", "ற", "ன",
    ];

    let mut consonant_vowels = Vec::new();
    for consonant in &consonants {
        for vowel_sign in &vowel_signs {
            consonant_vowels.push(format!("{}{}", consonant, vowel_sign));
        }
    }

    include_str!("./abbrev/ta.txt")
        .lines()
        .chain(include_str!("./abbrev/en.txt").lines())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect::<Vec<String>>()
        .into_iter()
        .chain(vowels.into_iter().map(String::from))
        .chain(consonants.into_iter().map(String::from))
        .chain(consonant_vowels)
        .collect()
});

impl Language for Tamil {
    fn get_abbreviations(&self) -> &[String] {
        &TAMIL_ABBREVIATIONS
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Tamil {}, "tests/ta.txt");
    }
}
