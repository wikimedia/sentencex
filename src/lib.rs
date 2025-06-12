use languages::{
    Amharic, Arabic, Bulgarian, English, Italian, Kannada, Kazakh, Language, Malayalam, Portuguese,
    Spanish, Tamil, Telegu,
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
    match current_code {
        // move the blow mapping to a static hashmap. AI!
        "en" => return Box::new(English {}),
        "es" => return Box::new(Spanish {}),
        "ml" => return Box::new(Malayalam {}),
        "pt" => return Box::new(Portuguese {}),
        "it" => return Box::new(Italian {}),
        "am" => return Box::new(Amharic {}),
        "ar" => return Box::new(Arabic {}),
        "ta" => return Box::new(Tamil {}),
        "te" => return Box::new(Telegu {}),
        "kn" => return Box::new(Kannada {}),
        "kk" => return Box::new(Kazakh {}),
        "bg" => return Box::new(Bulgarian {}),
        _ => {
            if let Some(fallbacks) = LANGUAGE_FALLBACKS.get(current_code) {
                for next_code in fallbacks {
                    let instance = language_factory(next_code);
                    return instance;
                }
            }
            return Box::new(English {}); // Default to English
        }
    }
}

pub fn segment(language_code: &str, text: &str) -> Vec<String> {
    let language = language_factory(language_code);
    language.segment(text)
}
