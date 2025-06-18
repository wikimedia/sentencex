use once_cell::sync::Lazy;

use super::Language;

#[derive(Debug, Clone)]
pub struct Japanese {}
static JAPANESE_ABBREVIATIONS: Lazy<Vec<String>> = Lazy::new(|| Vec::new());

impl Language for Japanese {
    fn get_abbreviations(&self) -> Vec<String> {
        JAPANESE_ABBREVIATIONS.clone()
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
