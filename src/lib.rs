use pyo3::prelude::*;

mod codemap;
mod repr_utils;
mod syntax;

/// A Python module implemented in Rust.
#[pymodule]
fn starlark_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<codemap::PyCodeMap>()?;
    m.add_class::<codemap::PyFileSpan>()?;
    m.add_class::<codemap::PyPos>()?;
    m.add_class::<codemap::PyResolvedFileLine>()?;
    m.add_class::<codemap::PyResolvedFileSpan>()?;
    m.add_class::<codemap::PyResolvedPos>()?;
    m.add_class::<codemap::PyResolvedSpan>()?;
    m.add_class::<codemap::PySpan>()?;
    m.add_class::<syntax::PyAstModule>()?;
    m.add_class::<syntax::PyDialect>()?;
    m.add_class::<syntax::PyDialectTypes>()?;
    Ok(())
}
