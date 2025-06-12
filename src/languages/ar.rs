use super::Language;

#[derive(Debug, Clone)]
pub struct Arabic {}

impl Language for Arabic {
    fn get_abbreviations(&self) -> Vec<String> {
        include_str!("./abbrev/ar.abbreviations.txt")
            .lines()
            .chain(include_str!("./abbrev/en.abbreviations.txt").lines())
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
        run_language_tests(Arabic {}, "tests/ar.txt");
    }
}
