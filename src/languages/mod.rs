mod am;
mod ar;
mod en;
mod es;
mod it;
mod kn;
mod language;
mod ml;
mod pt;
mod ta;

pub use am::Amharic;
pub use ar::Arabic;
pub use en::English;
pub use es::Spanish;
pub use it::Italian;
pub use kn::Kannada;
pub use language::Language;
pub use ml::Malayalam;
pub use pt::Portuguese;
pub use ta::Tamil;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    pub fn run_language_tests<T: Language>(language: T, test_file: &str) {
        let content = fs::read_to_string(test_file).expect("Failed to read test file");
        let test_cases: Vec<&str> = content.split("===\n").collect();

        for case in test_cases {
            if case.trim().starts_with('#') {
                continue; // Skip comment lines
            }
            let parts: Vec<&str> = case.split("---\n").collect();
            if parts.len() != 2 {
                continue; // Skip malformed test cases
            }

            let input = parts[0].trim();
            let expected: Vec<&str> = parts[1].lines().map(|line| line.trim()).collect();
            let result = language.segment(input);
            let trimmed_result: Vec<String> =
                result.iter().map(|item| item.trim().to_string()).collect();

            assert_eq!(trimmed_result, expected, "Failed for input: \n{}", input);
        }
    }
}
