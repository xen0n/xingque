use std::collections::HashMap;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use starlark::syntax::{AstLoad, AstModule, Dialect, DialectTypes};

use crate::codemap::{PyFileSpan, PySpan};
use crate::repr_utils::{PyReprBool, PyReprDialectTypes};

#[pyclass(module = "xingque", name = "DialectTypes")]
#[derive(PartialEq, Eq)]
pub(crate) enum PyDialectTypes {
    #[pyo3(name = "DISABLE")]
    Disable,
    #[pyo3(name = "PARSE_ONLY")]
    ParseOnly,
    #[pyo3(name = "ENABLE")]
    Enable,
}

#[pymethods]
impl PyDialectTypes {
    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let me: &Self = &slf.borrow();
        let desc: &str = me.into();
        Ok(format!("DialectTypes.{}", desc))
    }

    fn __str__(&self) -> &str {
        self.into()
    }

    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        if let Ok(other) = other.extract::<Self>() {
            *self == other
        } else if let Ok(other) = other.extract::<&str>() {
            if let Ok(other) = other.try_into() {
                *self == other
            } else {
                // conversion from str failed, meaning the value is invalid for
                // the type
                false
            }
        } else {
            false
        }
    }
}

impl TryFrom<&str> for PyDialectTypes {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "DISABLE" => Ok(Self::Disable),
            "PARSE_ONLY" => Ok(Self::ParseOnly),
            "ENABLE" => Ok(Self::Enable),
            _ => Err("invalid string value of DialectTypes"),
        }
    }
}

impl From<&PyDialectTypes> for &str {
    fn from(value: &PyDialectTypes) -> Self {
        match value {
            PyDialectTypes::Disable => "DISABLE",
            PyDialectTypes::ParseOnly => "PARSE_ONLY",
            PyDialectTypes::Enable => "ENABLE",
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

#[pyclass(module = "xingque", name = "Dialect")]
pub(crate) struct PyDialect(Dialect);

macro_rules! trivial_bool_prop {
    // still no concat_idents! so we have to duplicate a little
    // see https://github.com/rust-lang/rust/issues/29599
    ($cls: ident, $field: ident, $getter_name: ident, $setter_name: ident) => {
        #[pymethods]
        impl $cls {
            #[getter]
            fn $getter_name(&self) -> PyResult<bool> {
                Ok(self.0.$field)
            }

            #[setter]
            fn $setter_name(&mut self, value: bool) -> PyResult<()> {
                self.0.$field = value;
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
    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname()?;
        let me = slf.borrow();
        Ok(format!(
            "{}(enable_def={}, enable_lambda={}, enable_load={}, enable_keyword_only_arguments={}, enable_types={}, enable_load_reexport={}, enable_top_level_stmt={}, enable_f_strings={})",
            class_name,
            PyReprBool(me.0.enable_def),
            PyReprBool(me.0.enable_lambda),
            PyReprBool(me.0.enable_load),
            PyReprBool(me.0.enable_keyword_only_arguments),
            PyReprDialectTypes(me.0.enable_types),
            PyReprBool(me.0.enable_load_reexport),
            PyReprBool(me.0.enable_top_level_stmt),
            PyReprBool(me.0.enable_f_strings),
        ))
    }

    #[classattr]
    const EXTENDED: Self = Self(Dialect::Extended);

    #[classattr]
    const STANDARD: Self = Self(Dialect::Standard);

    #[getter]
    fn get_enable_types(&self) -> PyResult<PyDialectTypes> {
        Ok(self.0.enable_types.into())
    }

    #[setter]
    fn set_enable_types(&mut self, value: PyDialectTypes) -> PyResult<()> {
        self.0.enable_types = value.into();
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
        Ok(Dialect {
            enable_def: enable_def.unwrap_or(false),
            enable_lambda: enable_lambda.unwrap_or(false),
            enable_load: enable_load.unwrap_or(false),
            enable_keyword_only_arguments: enable_keyword_only_arguments.unwrap_or(false),
            enable_types: enable_types.map_or(DialectTypes::Disable, Into::into),
            enable_load_reexport: enable_load_reexport.unwrap_or(false),
            enable_top_level_stmt: enable_top_level_stmt.unwrap_or(false),
            enable_f_strings: enable_f_strings.unwrap_or(false),
            ..Dialect::default()
        }
        .into())
    }
}

impl From<Dialect> for PyDialect {
    fn from(value: Dialect) -> Self {
        Self(value)
    }
}

#[pyclass(module = "xingque", name = "AstModule")]
pub(crate) struct PyAstModule(AstModule);

impl From<AstModule> for PyAstModule {
    fn from(value: AstModule) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyAstModule {
    #[staticmethod]
    #[pyo3(signature = (path, dialect = &PyDialect::STANDARD))]
    fn parse_file(path: ::std::path::PathBuf, dialect: &PyDialect) -> PyResult<Self> {
        match AstModule::parse_file(&path, &dialect.0) {
            Ok(x) => Ok(x.into()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    #[pyo3(signature = (filename, content, dialect = &PyDialect::STANDARD))]
    fn parse(filename: &str, content: String, dialect: &PyDialect) -> PyResult<Self> {
        match AstModule::parse(filename, content, &dialect.0) {
            Ok(x) => Ok(x.into()),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    #[getter]
    fn loads(&self) -> Vec<PyAstLoad> {
        self.0.loads().into_iter().map(Into::into).collect()
    }

    fn file_span(&self, x: &PySpan) -> PyFileSpan {
        self.0.file_span(x.0).into()
    }

    #[getter]
    fn stmt_locations(&self) -> Vec<PyFileSpan> {
        self.0
            .stmt_locations()
            .into_iter()
            .map(Into::into)
            .collect()
    }

    fn replace_binary_operators(&mut self, replace: HashMap<String, String>) {
        self.0.replace_binary_operators(&replace);
    }
}

#[pyclass(module = "xingque", name = "AstLoad")]
pub(crate) struct PyAstLoad {
    /// Span where this load is written
    #[pyo3(get)]
    span: PyFileSpan,
    /// Module being loaded
    #[pyo3(get)]
    module_id: String,
    /// Symbols loaded from that module (local ident -> source ident)
    #[pyo3(get)]
    symbols: HashMap<String, String>,
}

impl<'py> From<AstLoad<'py>> for PyAstLoad {
    fn from(value: AstLoad<'py>) -> Self {
        Self {
            span: value.span.into(),
            module_id: value.module_id.to_string(),
            symbols: {
                let mut x: HashMap<String, String> = HashMap::new();
                for (k, v) in value.symbols.iter() {
                    x.insert(k.to_string(), v.to_string());
                }
                x
            },
        }
    }
}
