use super::Language;

#[derive(Debug, Clone)]
pub struct English {}

impl Language for English {
    fn get_abbreviations(&self) -> Vec<String> {
        include_str!("./abbrev/en.abbreviations.txt")
            .lines()
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
        run_language_tests(English {}, "tests/en.txt");
    }
}
