use pyo3::prelude::*;

mod codemap;
mod environment;
mod errors;
mod eval;
mod py2sl;
mod repr_utils;
mod sl2py;
mod syntax;
mod values;

#[pymodule]
mod xingque {
    use super::*;

    #[pymodule_export]
    use codemap::PyCodeMap;
    #[pymodule_export]
    use codemap::PyFileSpan;
    #[pymodule_export]
    use codemap::PyPos;
    #[pymodule_export]
    use codemap::PyResolvedFileLine;
    #[pymodule_export]
    use codemap::PyResolvedFileSpan;
    #[pymodule_export]
    use codemap::PyResolvedPos;
    #[pymodule_export]
    use codemap::PyResolvedSpan;
    #[pymodule_export]
    use codemap::PySpan;
    #[pymodule_export]
    use environment::PyFrozenModule;
    #[pymodule_export]
    use environment::PyGlobals;
    #[pymodule_export]
    use environment::PyGlobalsBuilder;
    #[pymodule_export]
    use environment::PyLibraryExtension;
    #[pymodule_export]
    use environment::PyModule;
    #[pymodule_export]
    use errors::PyFrame;
    #[pymodule_export]
    use eval::PyCallStack;
    #[pymodule_export]
    use eval::PyDictFileLoader;
    #[pymodule_export]
    use eval::PyEvaluator;
    #[pymodule_export]
    use eval::PyProfileMode;
    #[pymodule_export]
    use syntax::PyAstModule;
    #[pymodule_export]
    use syntax::PyDialect;
    #[pymodule_export]
    use syntax::PyDialectTypes;
    #[pymodule_export]
    use values::PyFrozenValue;
    #[pymodule_export]
    use values::PyHeap;
    #[pymodule_export]
    use values::PyHeapSummary;
    #[pymodule_export]
    use values::PyValue;

    #[pymodule_init]
    fn init(m: &Bound<'_, pyo3::types::PyModule>) -> PyResult<()> {
        m.add(
            "VERSION",
            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"),
        )?;
        m.add("STARLARK_RUST_VERSION", "0.12.0")?; // TODO: query this from Cargo
        Ok(())
    }
}
