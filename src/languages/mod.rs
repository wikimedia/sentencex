mod am;
mod ar;
mod bg;
mod bn;
mod ca;
mod da;
mod de;
mod el;
mod en;
mod es;
mod fi;
mod fr;
mod gu;
mod hi;
mod hy;
mod it;
mod ja;
mod kk;
mod kn;
mod language;
mod ml;
mod mr;
mod my;
mod nl;
mod pa;
mod pt;
mod ta;
mod te;
pub use am::Amharic;
pub use ar::Arabic;
pub use bg::Bulgarian;
pub use bn::Bengali;
pub use ca::Catalan;
pub use da::Danish;
pub use de::Deutch;
pub use el::Greek;
pub use en::English;
pub use es::Spanish;
pub use fi::Finnish;
pub use fr::French;
pub use gu::Gujarati;
pub use hi::Hindi;
pub use hy::Armenian;
pub use it::Italian;
pub use ja::Japanese;
pub use kk::Kazakh;
pub use kn::Kannada;
pub use language::Language;
pub use ml::Malayalam;
pub use mr::Marathi;
pub use my::Burmese;
pub use nl::Dutch;
pub use pa::Punjabi;
pub use pt::Portuguese;
pub use ta::Tamil;
pub use te::Telegu;

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
