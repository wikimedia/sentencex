use super::Language;

#[derive(Debug, Clone)]
pub struct Kazakh {}

impl Language for Kazakh {
    fn get_abbreviations(&self) -> Vec<String> {
        include_str!("./kk.abbreviations.txt")
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.starts_with("//") && !line.is_empty())
            .collect()
    }

    fn get_last_word<'a>(&self, text: &'a str) -> &'a str {
        let words: Vec<&str> = text
            .split(|c: char| c.is_whitespace() || c == '.')
            .filter(|word| !word.is_empty())
            .collect();

        if let Some(&last_word) = words.last() {
            return last_word;
        }

        ""
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        let regex = regex::Regex::new(r"^\W*[0-9a-zа-я]").unwrap();
        regex.is_match(text_after_boundary)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Kazakh {}, "tests/kk.txt");
    }
}
