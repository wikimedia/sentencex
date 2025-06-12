use pyo3::prelude::*;
use sentencex::segment;

#[pyclass]
struct SentenceSegmenter;

#[pymethods]
impl SentenceSegmenter {
    #[new]
    pub fn new() -> Self {
        SentenceSegmenter
    }

    pub fn segment(&self, language: &str, text: &str) -> Vec<String> {
        segment(language, text)
    }
}

#[pymodule]
fn bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SentenceSegmenter>()?;
    Ok(())
}
