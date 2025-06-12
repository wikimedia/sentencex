use super::Language;

#[derive(Debug, Clone)]
pub struct Italian {}

impl Language for Italian {
    fn get_abbreviations(&self) -> Vec<String> {
        include_str!("./abbrev/it.txt")
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.starts_with("//") && !line.is_empty())
            .collect()
    }

    fn is_punctuation_between_quotes(&self) -> bool {
        false
    }

    fn get_last_word<'a>(&self, text: &'a str) -> &'a str {
        let words: Vec<&str> = text
            .split(|c: char| c.is_whitespace() || c == '.')
            .collect();

        if words.is_empty() {
            return "";
        }

        let last_word = words[words.len() - 1];
        let parts: Vec<&str> = last_word.split("l'").collect();
        parts.last().unwrap_or(&"")
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        let regex = regex::Regex::new(r"^[0-9a-z]").unwrap();
        regex.is_match(text_after_boundary)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Italian {}, "tests/it.txt");
    }
}
