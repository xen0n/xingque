use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use starlark::environment::{Globals, LibraryExtension};
use starlark::values::FrozenStringValue;

/// The extra library definitions available in this Starlark implementation, but not in the standard.
#[pyclass(
    module = "xingque",
    name = "LibraryExtension",
    rename_all = "SCREAMING_SNAKE_CASE",
    frozen
)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PyLibraryExtension {
    /// Definitions to support the `struct` type, the `struct()` constructor.
    StructType,
    /// Definitions to support the `record` type, the `record()` constructor and `field()` function.
    RecordType,
    /// Definitions to support the `enum` type, the `enum()` constructor.
    EnumType,
    /// A function `map(f, xs)` which applies `f` to each element of `xs` and returns the result.
    Map,
    /// A function `filter(f, xs)` which applies `f` to each element of `xs` and returns those for which `f` returns `True`.
    /// As a special case, `filter(None, xs)` removes all `None` values.
    Filter,
    /// Partially apply a function, `partial(f, *args, **kwargs)` will create a function where those `args` `kwargs`
    /// are already applied to `f`.
    Partial,
    /// Add a function `debug(x)` which shows the Rust `Debug` representation of a value.
    /// Useful when debugging, but the output should not be considered stable.
    Debug,
    /// Add a function `print(x)` which prints to stderr.
    Print,
    /// Add a function `pprint(x)` which pretty-prints to stderr.
    Pprint,
    /// Add a function `breakpoint()` which will drop into a console-module evaluation prompt.
    Breakpoint,
    /// Add a function `json()` which will generate JSON for a module.
    Json,
    /// Provides `typing.All`, `typing.Callable` etc.
    /// Usually used in conjunction with
    /// `Dialect.enable_types`.
    Typing,
    /// Utilities exposing starlark-rust internals.
    /// These are not for production use.
    Internal,
    /// Add a function `call_stack()` which returns a string representation of
    /// the current call stack.
    CallStack,
    // NOTE: keep this in sync with LibraryExtension
}

impl From<LibraryExtension> for PyLibraryExtension {
    fn from(value: LibraryExtension) -> Self {
        match value {
            LibraryExtension::StructType => Self::StructType,
            LibraryExtension::RecordType => Self::RecordType,
            LibraryExtension::EnumType => Self::EnumType,
            LibraryExtension::Map => Self::Map,
            LibraryExtension::Filter => Self::Filter,
            LibraryExtension::Partial => Self::Partial,
            LibraryExtension::Debug => Self::Debug,
            LibraryExtension::Print => Self::Print,
            LibraryExtension::Pprint => Self::Pprint,
            LibraryExtension::Breakpoint => Self::Breakpoint,
            LibraryExtension::Json => Self::Json,
            LibraryExtension::Typing => Self::Typing,
            LibraryExtension::Internal => Self::Internal,
            LibraryExtension::CallStack => Self::CallStack,
        }
    }
}

impl From<PyLibraryExtension> for LibraryExtension {
    fn from(value: PyLibraryExtension) -> Self {
        match value {
            PyLibraryExtension::StructType => Self::StructType,
            PyLibraryExtension::RecordType => Self::RecordType,
            PyLibraryExtension::EnumType => Self::EnumType,
            PyLibraryExtension::Map => Self::Map,
            PyLibraryExtension::Filter => Self::Filter,
            PyLibraryExtension::Partial => Self::Partial,
            PyLibraryExtension::Debug => Self::Debug,
            PyLibraryExtension::Print => Self::Print,
            PyLibraryExtension::Pprint => Self::Pprint,
            PyLibraryExtension::Breakpoint => Self::Breakpoint,
            PyLibraryExtension::Json => Self::Json,
            PyLibraryExtension::Typing => Self::Typing,
            PyLibraryExtension::Internal => Self::Internal,
            PyLibraryExtension::CallStack => Self::CallStack,
        }
    }
}

#[pymethods]
impl PyLibraryExtension {
    fn __hash__(&self) -> u64 {
        use std::hash::Hash;
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

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

    #[staticmethod]
    fn extended_by(extensions: &Bound<'_, PyAny>) -> PyResult<Self> {
        let extensions = {
            let mut tmp = Vec::new();
            for x in extensions.iter()? {
                match x {
                    Ok(x) => match x.extract::<PyLibraryExtension>() {
                        Ok(x) => tmp.push(x.into()),
                        Err(e) => return Err(PyValueError::new_err(e)),
                    },
                    Err(e) => return Err(PyValueError::new_err(e)),
                }
            }
            tmp
        };
        Ok(Globals::extended_by(&extensions).into())
    }

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
