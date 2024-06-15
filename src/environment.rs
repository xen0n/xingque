use pyo3::prelude::*;
use starlark::environment::Globals;
use starlark::values::FrozenStringValue;

#[pyclass(module = "xingque", name = "Globals")]
pub(crate) struct PyGlobals(Globals);

impl From<Globals> for PyGlobals {
    fn from(value: Globals) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyGlobals {
    #[new]
    fn new() -> Self {
        Globals::new().into()
    }

    #[staticmethod]
    fn standard() -> Self {
        Globals::standard().into()
    }

    // TODO: extended_by

    #[getter]
    fn names(slf: &Bound<'_, Self>) -> PyResult<Py<PyGlobalsNamesIterator>> {
        Py::new(
            slf.py(),
            PyGlobalsNamesIterator::new(slf, Box::new(slf.borrow().0.names())),
        )
    }

    // TODO: iter

    fn describe(&self) -> String {
        self.0.describe()
    }

    #[getter]
    fn docstring(&self) -> Option<&str> {
        self.0.docstring()
    }

    // TODO: documentation
}

// TODO: is the unsendable marker removable?
#[pyclass(module = "xingque", name = "_GlobalsNamesIterator", unsendable)]
pub(crate) struct PyGlobalsNamesIterator {
    _parent: Py<PyGlobals>,
    inner: Box<dyn Iterator<Item = FrozenStringValue>>,
}

impl PyGlobalsNamesIterator {
    fn new(
        parent: &Bound<'_, PyGlobals>,
        value: Box<dyn Iterator<Item = FrozenStringValue> + '_>,
    ) -> Self {
        let parent = parent.clone().unbind();
        Self {
            _parent: parent,
            // Safety: parent is kept alive by the reference above
            inner: unsafe { ::core::mem::transmute(value) },
        }
    }
}

#[pymethods]
impl PyGlobalsNamesIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<&str> {
        slf.inner.next().map(|x| x.as_str())
    }
}
