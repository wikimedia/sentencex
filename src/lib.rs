use languages::{
    Amharic, Arabic, Bulgarian, English, Italian, Kannada, Kazakh, Language, Malayalam, Portuguese,
    Spanish, Tamil,
};
use serde::Serialize;

mod constants;
pub mod languages;

#[derive(Debug, Clone)]
pub struct SentenceSegmenter {
    language: LanguageOption,
}

#[derive(Debug, Clone, Serialize, Default, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum LanguageOption {
    #[default]
    English,
    Spanish,
    Malayalam,
    Portuguese,
    Italian,
    Amharic,
    Arabic,
    Tamil,
    Kannada,
    Kazakh,
    Bulgarian,
}

use crate::languages::*;
use crate::languages::fallbacks::LANGUAGE_FALLBACKS;

fn language_factory(language_code: &str) -> Box<dyn Language> {
    let mut current_code = language_code;

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

impl SentenceSegmenter {
    pub fn new(language: LanguageOption) -> Self {
        SentenceSegmenter { language }
    }

    pub fn segment(&self, text: &str) -> Vec<String> {
        match self.language {
            LanguageOption::English => English {}.segment(text),
            LanguageOption::Spanish => Spanish {}.segment(text),
            LanguageOption::Malayalam => Malayalam {}.segment(text),
            LanguageOption::Portuguese => Portuguese {}.segment(text),
            LanguageOption::Italian => Italian {}.segment(text),
            LanguageOption::Amharic => Amharic {}.segment(text),
            LanguageOption::Arabic => Arabic {}.segment(text),
            LanguageOption::Tamil => Tamil {}.segment(text),
            LanguageOption::Kannada => Kannada {}.segment(text),
            LanguageOption::Kazakh => Kazakh {}.segment(text),
            LanguageOption::Bulgarian => Bulgarian {}.segment(text),
        }
    }
}
