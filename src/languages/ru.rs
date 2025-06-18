use super::Language;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Russian {}

static ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("./abbrev/ru.txt")
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .collect()
});
impl Language for Russian {
    fn get_abbreviations(&self) -> Vec<String> {
        ABBREVIATIONS.clone()
    }

    fn continue_in_next_word(&self, text_after_boundary: &str) -> bool {
        //rewrite below js code in Rust. AI!
return textAfterBoundary.match(/^[0-9a-zа-я]/)
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Russian {}, "tests/ru.txt");
    }
}
