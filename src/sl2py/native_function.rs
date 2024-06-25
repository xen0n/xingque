use pyo3::prelude::*;
use starlark::values::function::NativeFunction;

#[pyclass(module = "xingque", name = "_SlNativeFunction", frozen)]
pub(crate) struct PySlNativeFunction(&'static NativeFunction);

impl PySlNativeFunction {
    pub(crate) fn new(value: &NativeFunction) -> Self {
        // Safety: Rust functions will live as long as the extension is loaded
        Self(unsafe { ::core::mem::transmute(value) })
    }

    pub(crate) fn new_py_any(py: Python, value: &NativeFunction) -> PyResult<PyObject> {
        Py::new(py, Self::new(value)).map(Py::into_any)
    }
}

#[pymethods]
impl PySlNativeFunction {
    fn __repr__(&self) -> String {
        format!("<Starlark native fn {}>", self.0.to_string())
    }

    // this isn't directly callable because an Evaluator is needed
}
