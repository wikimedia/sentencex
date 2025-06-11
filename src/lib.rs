use languages::{English, Italian, Language, Malayalam, Portuguese, Spanish};
use pyo3::prelude::*;

mod constants;
mod languages;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SentenceSegmenter {
    language: LanguageOption,
}

#[derive(Debug, Clone)]
#[pyclass]
pub enum LanguageOption {
    English,
    Spanish,
    Malayalam,
    Portuguese,
    Italian,
}

#[pymethods]
impl SentenceSegmenter {
    #[new]
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
        }
    }
}

#[pymodule]
fn sentencex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SentenceSegmenter>()?;
    m.add_class::<LanguageOption>()?;
    Ok(())
}
