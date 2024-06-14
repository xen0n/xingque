use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use starlark::syntax::{AstModule, Dialect, DialectTypes};

#[pyclass(module = "starlark_pyo3", name = "DialectTypes")]
pub(crate) enum PyDialectTypes {
    #[pyo3(name = "DISABLE")]
    Disable,
    #[pyo3(name = "PARSE_ONLY")]
    ParseOnly,
    #[pyo3(name = "ENABLE")]
    Enable,
}

impl TryFrom<&str> for PyDialectTypes {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "disable" | "Disable" | "DISABLE" => Ok(Self::Disable),
            "parse-only" | "parse_only" | "ParseOnly" | "PARSE_ONLY" => Ok(Self::ParseOnly),
            "enable" | "Enable" | "ENABLE" => Ok(Self::Enable),
            _ => Err("invalid string value of DialectTypes"),
        }
    }
}

impl From<&PyDialectTypes> for &str {
    fn from(value: &PyDialectTypes) -> Self {
        match value {
            PyDialectTypes::Disable => "disable",
            PyDialectTypes::ParseOnly => "parse-only",
            PyDialectTypes::Enable => "enable",
        }
    }
}

impl<'py> FromPyObject<'py> for PyDialectTypes {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s = ob.downcast::<PyString>()?;
        s.to_str()?.try_into().map_err(PyValueError::new_err)
    }
}

impl ToPyObject for PyDialectTypes {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let x: &str = self.into();
        x.to_object(py)
    }
}

impl From<PyDialectTypes> for DialectTypes {
    fn from(value: PyDialectTypes) -> Self {
        match value {
            PyDialectTypes::Disable => Self::Disable,
            PyDialectTypes::ParseOnly => Self::ParseOnly,
            PyDialectTypes::Enable => Self::Enable,
        }
    }
}

impl From<DialectTypes> for PyDialectTypes {
    fn from(value: DialectTypes) -> Self {
        match value {
            DialectTypes::Disable => Self::Disable,
            DialectTypes::ParseOnly => Self::ParseOnly,
            DialectTypes::Enable => Self::Enable,
        }
    }
}

#[pyclass(module = "starlark_pyo3", name = "Dialect")]
pub(crate) struct PyDialect {
    inner: Dialect,
}

macro_rules! trivial_bool_prop {
    // still no concat_idents! so we have to duplicate a little
    // see https://github.com/rust-lang/rust/issues/29599
    ($cls: ident, $field: ident, $getter_name: ident, $setter_name: ident) => {
        #[pymethods]
        impl $cls {
            #[getter]
            fn $getter_name(&self) -> PyResult<bool> {
                Ok(self.inner.$field)
            }

            #[setter]
            fn $setter_name(&mut self, value: bool) -> PyResult<()> {
                self.inner.$field = value;
                Ok(())
            }
        }
    };
}

trivial_bool_prop!(PyDialect, enable_def, get_enable_def, set_enable_def);
trivial_bool_prop!(
    PyDialect,
    enable_lambda,
    get_enable_lambda,
    set_enable_lambda
);
trivial_bool_prop!(PyDialect, enable_load, get_enable_load, set_enable_load);
trivial_bool_prop!(
    PyDialect,
    enable_keyword_only_arguments,
    get_enable_keyword_only_arguments,
    set_enable_keyword_only_arguments
);
trivial_bool_prop!(
    PyDialect,
    enable_load_reexport,
    get_enable_load_reexport,
    set_enable_load_reexport
);
trivial_bool_prop!(
    PyDialect,
    enable_top_level_stmt,
    get_enable_top_level_stmt,
    set_enable_top_level_stmt
);
trivial_bool_prop!(
    PyDialect,
    enable_f_strings,
    get_enable_f_strings,
    set_enable_f_strings
);

#[pymethods]
impl PyDialect {
    #[classattr]
    const EXTENDED: Self = Self {
        inner: Dialect::Extended,
    };
    #[classattr]
    const STANDARD: Self = Self {
        inner: Dialect::Standard,
    };

    #[getter]
    fn get_enable_types(&self) -> PyResult<PyDialectTypes> {
        Ok(self.inner.enable_types.into())
    }

    #[setter]
    fn set_enable_types(&mut self, value: PyDialectTypes) -> PyResult<()> {
        self.inner.enable_types = value.into();
        Ok(())
    }

    #[new]
    fn py_new(
        enable_def: Option<bool>,
        enable_lambda: Option<bool>,
        enable_load: Option<bool>,
        enable_keyword_only_arguments: Option<bool>,
        enable_types: Option<PyDialectTypes>,
        enable_load_reexport: Option<bool>,
        enable_top_level_stmt: Option<bool>,
        enable_f_strings: Option<bool>,
    ) -> PyResult<Self> {
        let inner = Dialect {
            enable_def: enable_def.unwrap_or(false),
            enable_lambda: enable_lambda.unwrap_or(false),
            enable_load: enable_load.unwrap_or(false),
            enable_keyword_only_arguments: enable_keyword_only_arguments.unwrap_or(false),
            enable_types: enable_types.map_or(DialectTypes::Disable, Into::into),
            enable_load_reexport: enable_load_reexport.unwrap_or(false),
            enable_top_level_stmt: enable_top_level_stmt.unwrap_or(false),
            enable_f_strings: enable_f_strings.unwrap_or(false),
            ..Dialect::default()
        };
        Ok(Self::new(inner))
    }
}

impl PyDialect {
    fn new(inner: Dialect) -> Self {
        Self { inner }
    }
}

#[pyclass(module = "starlark_pyo3", name = "AstModule")]
pub(crate) struct PyAstModule {
    inner: AstModule,
}

#[pymethods]
impl PyAstModule {
    #[staticmethod]
    #[pyo3(signature = (path, dialect = &PyDialect::STANDARD))]
    fn parse_file(path: ::std::path::PathBuf, dialect: &PyDialect) -> PyResult<Self> {
        match AstModule::parse_file(&path, &dialect.inner) {
            Ok(inner) => Ok(Self { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    #[pyo3(signature = (filename, content, dialect = &PyDialect::STANDARD))]
    fn parse(filename: &str, content: String, dialect: &PyDialect) -> PyResult<Self> {
        match AstModule::parse(filename, content, &dialect.inner) {
            Ok(inner) => Ok(Self { inner }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }
}
