use super::Language;

#[derive(Debug, Clone)]
pub struct Dutch {}

impl Language for Dutch {
    fn get_abbreviations(&self) -> Vec<String> {
        include_str!("./abbrev/nl.txt")
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
        run_language_tests(Dutch {}, "tests/nl.txt");
    }
}
