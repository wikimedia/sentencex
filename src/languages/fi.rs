use super::Language;

#[derive(Debug, Clone)]
pub struct Finnish {}

impl Language for Finnish {
    fn get_abbreviations(&self) -> Vec<String> {
        include_str!("./abbrev/it.txt")
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.starts_with("//") && !line.is_empty())
            .collect()
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        use regex::Regex;

        // Check if the text matches the regex pattern
        let regex = Regex::new(r"^\W*[0-9a-z]").unwrap();
        if regex.is_match(text_after_boundary) {
            return true;
        }

        // Extract the next word
        let next_word = text_after_boundary
            .trim()
            .split_whitespace()
            .next()
            .unwrap_or("");

        // Check conditions for the next word
        if next_word.is_empty() {
            return false;
        }

        let months = vec![
            "January", "February", "March", "April", "May", "June", "July", "August", "September",
            "October", "November", "December",
        ];

        if months.contains(&next_word)
            || months.contains(&format!(
                "{}{}",
                next_word.chars().next().unwrap_or_default().to_uppercase(),
                &next_word[1..]
            ))
        {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Finnish {}, "tests/fi.txt");
    }
}
