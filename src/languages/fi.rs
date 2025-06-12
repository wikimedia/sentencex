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
        // Rewrite the below python code in Rust. AI!
        if re.match(r"^\W*[0-9a-z]", text_after_boundary):
            return True
        next_word = text_after_boundary.strip().split(" ")[0]

        if len(next_word) == 0:
            return False
        if next_word in self.MONTHS or (next_word[0].upper() + next_word[1:] in self.MONTHS):
            return True
        return False
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
