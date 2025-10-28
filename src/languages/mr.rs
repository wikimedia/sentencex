use super::{Hindi, Language};

#[derive(Debug, Clone)]
pub struct Marathi {}

impl Language for Marathi {
    fn get_abbreviations(&self) -> &[String] {
        Hindi {}.get_abbreviations()
    }
}

#[cfg(test)]
mod tests {
    use crate::languages::tests::run_language_tests;

    use super::*;

    #[test]
    fn test_segment() {
        run_language_tests(Marathi {}, "tests/mr.txt");
    }
}
