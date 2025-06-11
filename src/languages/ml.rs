use super::Language;

#[derive(Debug, Clone)]
pub struct Malayalam {}

impl Language for Malayalam {
    fn get_abbreviations(&self) -> Vec<String> {
        // Use ./en.abbreviations.txt and ./ml.abbreviations.txt. AI!
        include_str!("./ml.abbreviations.txt")
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
        run_language_tests(Malayalam {}, "tests/malayalam.txt");
    }
}
