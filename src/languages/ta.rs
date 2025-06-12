use super::Language;

#[derive(Debug, Clone)]
pub struct Tamil {}

impl Language for Tamil {
    fn get_abbreviations(&self) -> Vec<String> {
        let vowel_signs = vec!["ா", "ி", "ீ", "ு", "ூ", "ெ", "ே", "ை", "ொ", "ோ", "ௌ"];
        let vowels = vec!["அ", "ஆ", "இ", "ஈ", "உ", "ஊ", "எ", "ஏ", "ஐ", "ஒ", "ஓ", "ஔ"];
        let consonants = vec![
            "க", "ங", "ச", "ஞ", "ட", "ண", "த", "ந", "ப", "ம", "ய", "ர", "ல", "வ", "ழ", "ள", "ற",
            "ன",
        ];

        let mut consonant_vowels = Vec::new();
        for consonant in &consonants {
            for vowel_sign in &vowel_signs {
                consonant_vowels.push(format!("{}{}", consonant, vowel_sign));
            }
        }

        include_str!("./ta.abbreviations.txt")
            .lines()
            .chain(include_str!("./en.abbreviations.txt").lines())
            .map(|line| line.trim().to_string())
            .filter(|line| !line.starts_with("//") && !line.is_empty())
            .collect()
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
