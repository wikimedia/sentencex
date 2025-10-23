use ::sentencex::{get_sentence_boundaries as _get_sentence_boundaries, segment as _segment};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn segment(language: &str, text: &str) -> Vec<String> {
    _segment(language, text)
}

#[pyfunction]
pub fn get_sentence_boundaries(py: Python, language: &str, text: &str) -> PyResult<Vec<PyObject>> {
    let boundaries = _get_sentence_boundaries(language, text);

    let mut result = Vec::new();
    for boundary in boundaries {
        let dict = PyDict::new(py);
        dict.set_item("start_index", boundary.start_index)?;
        dict.set_item("end_index", boundary.end_index)?;
        dict.set_item("text", boundary.text)?;
        dict.set_item("boundary_symbol", boundary.boundary_symbol)?;
        dict.set_item("is_paragraph_break", boundary.is_paragraph_break)?;
        result.push(dict.into());
    }

    Ok(result)
}

#[pymodule]
fn sentencex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(segment))?;
    m.add_wrapped(wrap_pyfunction!(get_sentence_boundaries))?;
    Ok(())
}
