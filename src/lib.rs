use languages::{
    Amharic, Arabic, English, Italian, Kannada, Kazakh, Language, Malayalam, Portuguese, Spanish,
    Tamil,
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
        }
    }
}
