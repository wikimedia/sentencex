use super::Language;

#[derive(Debug, Clone)]
pub struct Japanese {}

impl Language for Japanese {
    fn get_abbreviations(&self) -> Vec<String> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Japanese {}, "tests/ja.txt");
    }
}
