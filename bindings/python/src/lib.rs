use ::sentencex::segment as _segment;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn segment(language: &str, text: &str) -> Vec<String> {
    _segment(language, text)
}

#[pymodule]
fn sentencex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(segment))?;
    Ok(())
}
