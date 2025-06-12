use pyo3::prelude::*;
use sentencex::segment;


# Fix the below export of rust function to python. AI!
#[pymethods]
pub fn segment(&self, language: &str, text: &str) -> Vec<String> {
    segment(language, text)
}
