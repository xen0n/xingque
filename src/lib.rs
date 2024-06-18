use pyo3::prelude::*;

mod codemap;
mod environment;
mod eval;
mod hash_utils;
mod py2sl;
mod repr_utils;
mod sl2py;
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
    m.add_class::<environment::PyFrozenModule>()?;
    m.add_class::<environment::PyGlobals>()?;
    m.add_class::<environment::PyGlobalsBuilder>()?;
    m.add_class::<environment::PyLibraryExtension>()?;
    m.add_class::<environment::PyModule>()?;
    m.add_class::<eval::PyDictFileLoader>()?;
    m.add_class::<eval::PyEvaluator>()?;
    m.add_class::<syntax::PyAstModule>()?;
    m.add_class::<syntax::PyDialect>()?;
    m.add_class::<syntax::PyDialectTypes>()?;
    m.add_class::<values::PyFrozenValue>()?;
    m.add_class::<values::PyHeap>()?;
    m.add_class::<values::PyHeapSummary>()?;
    m.add_class::<values::PyValue>()?;
    Ok(())
}
