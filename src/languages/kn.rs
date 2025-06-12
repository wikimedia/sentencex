use super::Language;

#[derive(Debug, Clone)]
pub struct Kannada {}

impl Language for Kannada {
    fn get_abbreviations(&self) -> Vec<String> {
        include_str!("./abbrev/kn.txt")
            .lines()
            .chain(include_str!("./abbrev/en.txt").lines())
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
        run_language_tests(Kannada {}, "tests/kn.txt");
    }
}
