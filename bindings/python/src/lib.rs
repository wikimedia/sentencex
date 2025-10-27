#[pyo3::pymodule(gil_used = false)]
mod sentencex {
    use ::sentencex::{get_sentence_boundaries as _get_sentence_boundaries, segment as _segment};
    use pyo3::{prelude::*, types::PyDict};

    #[pyfunction]
    pub fn segment(language: &str, text: &str) -> Vec<String> {
        _segment(language, text)
    }

    #[pyfunction]
    pub fn get_sentence_boundaries(
        py: Python,
        language: &str,
        text: &str,
    ) -> PyResult<Vec<Py<PyAny>>> {
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
}
