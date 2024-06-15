use pyo3::prelude::*;

mod codemap;
mod environment;
mod repr_utils;
mod syntax;
mod values;

#[pymodule]
fn xingque(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<codemap::PyCodeMap>()?;
    m.add_class::<codemap::PyFileSpan>()?;
    m.add_class::<codemap::PyPos>()?;
    m.add_class::<codemap::PyResolvedFileLine>()?;
    m.add_class::<codemap::PyResolvedFileSpan>()?;
    m.add_class::<codemap::PyResolvedPos>()?;
    m.add_class::<codemap::PyResolvedSpan>()?;
    m.add_class::<codemap::PySpan>()?;
    m.add_class::<environment::PyGlobals>()?;
    m.add_class::<syntax::PyAstModule>()?;
    m.add_class::<syntax::PyDialect>()?;
    m.add_class::<syntax::PyDialectTypes>()?;
    m.add_class::<values::PyHeap>()?;
    m.add_class::<values::PyHeapSummary>()?;
    Ok(())
}
