use languages::{
    Amharic, Arabic, Bulgarian, English, Italian, Kannada, Kazakh, LANGUAGE_FALLBACKS, Language,
    Malayalam, Portuguese, Spanish, Tamil,
};

mod constants;
pub mod languages;

fn language_factory(language_code: &str) -> Box<dyn Language> {
    let mut current_code = language_code;
    // READ languages/fallbacks.yaml (using include! syntax) to LANGUAGE_FALLBACKS which is
    // a hashmap with key as string, value as array of strings. AI!
    loop {
        match current_code {
            "en" => return Box::new(English {}),
            "es" => return Box::new(Spanish {}),
            "ml" => return Box::new(Malayalam {}),
            "pt" => return Box::new(Portuguese {}),
            "it" => return Box::new(Italian {}),
            "am" => return Box::new(Amharic {}),
            "ar" => return Box::new(Arabic {}),
            "ta" => return Box::new(Tamil {}),
            "kn" => return Box::new(Kannada {}),
            "kk" => return Box::new(Kazakh {}),
            "bg" => return Box::new(Bulgarian {}),
            _ => {
                if let Some(fallbacks) = LANGUAGE_FALLBACKS.get(current_code) {
                    if let Some(next_code) = fallbacks.first() {
                        current_code = next_code;
                        continue;
                    }
                }
                return Box::new(English {}); // Default to English
            }
        }
    }
}

pub fn segment(language_code: &str, text: &str) -> Vec<String> {
    let language = language_factory(language_code);
    language.segment(text)
}
