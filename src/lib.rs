use languages::{
    Amharic, Arabic, Armenian, Bengali, Bulgarian, Burmese, Catalan, Danish, Deutch, Dutch,
    English, Finnish, French, Greek, Gujarati, Hindi, Italian, Japanese, Kannada, Kazakh, Language,
    Malayalam, Marathi, Portuguese, Spanish, Tamil,
};

mod constants;
pub mod languages;

use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref LANGUAGE_FALLBACKS: HashMap<&'static str, Vec<&'static str>> = {
        let yaml_data = include_str!("./languages/fallbacks.yaml");
        serde_yaml::from_str(yaml_data).expect("Failed to parse fallbacks.yaml")
    };
}
fn language_factory(language_code: &str) -> Box<dyn Language> {
    let mut current_code = language_code;
    let mut visited = std::collections::HashSet::new();

    loop {
        if visited.contains(current_code) {
            current_code = "en"; // Default to English if a cycle is detected
        } else {
            visited.insert(current_code);
        }

        match current_code {
            "am" => return Box::new(Amharic {}),
            "ar" => return Box::new(Arabic {}),
            "bg" => return Box::new(Bulgarian {}),
            "bn" => return Box::new(Bengali {}),
            "ca" => return Box::new(Catalan {}),
            "da" => return Box::new(Danish {}),
            "de" => return Box::new(Deutch {}),
            "en" => return Box::new(English {}),
            "es" => return Box::new(Spanish {}),
            "el" => return Box::new(Greek {}),
            "gu" => return Box::new(Gujarati {}),
            "hi" => return Box::new(Hindi {}),
            "hy" => return Box::new(Armenian {}),
            "ja" => return Box::new(Japanese {}),
            "ml" => return Box::new(Malayalam {}),
            "mr" => return Box::new(Marathi {}),
            "my" => return Box::new(Burmese {}),
            "nl" => return Box::new(Dutch {}),
            "pt" => return Box::new(Portuguese {}),
            "it" => return Box::new(Italian {}),
            "ta" => return Box::new(Tamil {}),
            "te" => return Box::new(Tamil {}),
            "kn" => return Box::new(Kannada {}),
            "kk" => return Box::new(Kazakh {}),
            "fr" => return Box::new(French {}),
            "fi" => return Box::new(Finnish {}),
            _ => {
                if let Some(fallbacks) = LANGUAGE_FALLBACKS.get(current_code) {
                    for next_code in fallbacks {
                        if !visited.contains(next_code) {
                            current_code = next_code;
                            break;
                        }
                    }
                } else {
                    current_code = "en"; // Default to English if no fallbacks are found
                }
            }
        }
    }
}

pub fn segment(language_code: &str, text: &str) -> Vec<String> {
    let language = language_factory(language_code);
    language.segment(text)
}
