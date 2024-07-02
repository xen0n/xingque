use pyo3::prelude::*;
use starlark::codemap::FileSpan;
use starlark::errors::Frame;

use crate::codemap::PyFileSpan;

#[pyclass(module = "xingque", name = "Frame", frozen)]
#[derive(Clone)]
pub(crate) struct PyFrame(Frame);

impl From<Frame> for PyFrame {
    fn from(value: Frame) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyFrame {
    #[getter]
    fn name(&self) -> &str {
        &self.0.name
    }

    #[getter]
    fn location(&self) -> Option<PyFileSpan> {
        self.0.location.clone().map(FileSpan::into)
    }

    // TODO: write_two_lines
}
