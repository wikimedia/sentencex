use languages::{Amharic, English, Italian, Language, Malayalam, Portuguese, Spanish};

mod constants;
pub mod languages;

#[derive(Debug, Clone)]
pub struct SentenceSegmenter {
    language: LanguageOption,
}

#[derive(Debug, Clone)]
pub enum LanguageOption {
    English,
    Spanish,
    Malayalam,
    Portuguese,
    Italian,
    Amharic,
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
        }
    }
}
