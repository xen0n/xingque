use pyo3::prelude::*;

mod syntax;

/// A Python module implemented in Rust.
#[pymodule]
fn python_starlark_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<syntax::PyAstModule>()?;
    m.add_class::<syntax::PyDialect>()?;
    m.add_class::<syntax::PyDialectTypes>()?;
    Ok(())
}
