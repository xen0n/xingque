use pyo3::{exceptions::PyValueError, prelude::*};
use starlark::codemap::{Pos, Span};

#[pyclass(module = "starlark_pyo3", name = "Pos")]
pub(crate) struct PyPos(Pos);

#[pymethods]
impl PyPos {
    #[new]
    fn py_new(x: u32) -> Self {
        Self(Pos::new(x))
    }

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname()?;
        Ok(format!("{}({})", class_name, slf.borrow().get()))
    }

    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        if let Ok(other) = other.downcast::<PyPos>() {
            self.0 == other.borrow().0
        } else if let Ok(other) = other.extract::<u32>() {
            self.get() == other
        } else {
            false
        }
    }

    fn get(&self) -> u32 {
        self.0.get()
    }

    fn __int__(&self) -> u32 {
        self.get()
    }

    fn __add__(&self, other: u32) -> Self {
        Self(self.0 + other)
    }

    fn __iadd__(&mut self, other: u32) {
        self.0 += other;
    }
}

impl<'py> FromPyObject<'py> for PyPos {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self::py_new(ob.extract()?))
    }
}

#[pyclass(module = "starlark_pyo3", name = "Span")]
pub(crate) struct PySpan(Span);

#[pymethods]
impl PySpan {
    #[new]
    fn py_new(begin: PyPos, end: PyPos) -> Self {
        Self(Span::new(begin.0, end.0))
    }

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname()?;
        let me = slf.borrow();
        Ok(format!(
            "{}({}, {})",
            class_name,
            me.0.begin().get(),
            me.0.end().get()
        ))
    }

    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        match other.downcast::<PySpan>() {
            Ok(other) => self.0 == other.borrow().0,
            Err(_) => false,
        }
    }

    #[getter]
    fn get_begin(&self) -> PyPos {
        PyPos(self.0.begin())
    }

    #[getter]
    fn get_end(&self) -> PyPos {
        PyPos(self.0.end())
    }

    fn merge(&self, other: &Self) -> Self {
        Self(self.0.merge(other.0))
    }

    fn merge_all(&self) -> Self {
        // TODO: accept an Iterable
        todo!();
    }

    fn end_span(&self) -> Self {
        Self(self.0.end_span())
    }

    fn __contains__(&self, pos: &Bound<'_, PyAny>) -> PyResult<bool> {
        if let Ok(pos) = pos.downcast::<PyPos>() {
            Ok(self.0.contains(pos.borrow().0))
        } else if let Ok(pos) = pos.extract::<u32>() {
            Ok(self.0.contains(Pos::new(pos)))
        } else {
            Err(PyValueError::new_err(
                "invalid operand type for Span.__contains__",
            ))
        }
    }

    fn contains(&self, pos: &Bound<'_, PyAny>) -> PyResult<bool> {
        self.__contains__(pos)
    }
}
