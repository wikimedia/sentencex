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

lazy_static::lazy_static! {
    static ref LANGUAGE_MAP: HashMap<&'static str, fn() -> Box<dyn Language>> = {
        let mut map = HashMap::new();
        map.insert("en", || Box::new(English {}));
        map.insert("es", || Box::new(Spanish {}));
        map.insert("ml", || Box::new(Malayalam {}));
        map.insert("pt", || Box::new(Portuguese {}));
        map.insert("it", || Box::new(Italian {}));
        map.insert("am", || Box::new(Amharic {}));
        map.insert("ar", || Box::new(Arabic {}));
        map.insert("ta", || Box::new(Tamil {}));
        map.insert("te", || Box::new(Telegu {}));
        map.insert("kn", || Box::new(Kannada {}));
        map.insert("kk", || Box::new(Kazakh {}));
        map.insert("bg", || Box::new(Bulgarian {}));
        map
    };
}

fn language_factory(language_code: &str) -> Box<dyn Language> {
    if let Some(language_constructor) = LANGUAGE_MAP.get(language_code) {
        return language_constructor();
    }

    if let Some(fallbacks) = LANGUAGE_FALLBACKS.get(language_code) {
        for next_code in fallbacks {
            let instance = language_factory(next_code);
            return instance;
        }
    }

    Box::new(English {}) // Default to English
}

pub fn segment(language_code: &str, text: &str) -> Vec<String> {
    let language = language_factory(language_code);
    language.segment(text)
}
