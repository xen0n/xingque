use std::borrow::Cow;

use pyo3::{exceptions::PyValueError, prelude::*};
use starlark::codemap::{
    CodeMap, FileSpan, Pos, ResolvedFileLine, ResolvedFileSpan, ResolvedPos, ResolvedSpan, Span,
};

#[pyclass(module = "xingque", name = "Pos")]
pub(crate) struct PyPos(Pos);

#[pymethods]
impl PyPos {
    #[new]
    fn py_new(x: u32) -> Self {
        Self(Pos::new(x))
    }

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname()?;
        Ok(format!("{}({})", class_name, slf.borrow().get()))
    }

    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        if let Ok(other) = other.downcast::<PyPos>() {
            self.0 == other.borrow().0
        } else if let Ok(other) = other.extract::<u32>() {
            self.get() == other
        } else {
            false
        }
    }

    fn get(&self) -> u32 {
        self.0.get()
    }

    fn __int__(&self) -> u32 {
        self.get()
    }

    fn __add__(&self, other: u32) -> Self {
        Self(self.0 + other)
    }

    fn __iadd__(&mut self, other: u32) {
        self.0 += other;
    }
}

impl<'py> FromPyObject<'py> for PyPos {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self::py_new(ob.extract()?))
    }
}

#[pyclass(module = "xingque", name = "ResolvedPos", eq, hash, frozen)]
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) struct PyResolvedPos(ResolvedPos);

impl From<ResolvedPos> for PyResolvedPos {
    fn from(value: ResolvedPos) -> Self {
        Self(value)
    }
}

impl PyResolvedPos {
    fn repr(&self, class_name: Option<Cow<'_, str>>) -> String {
        format!(
            "{}(line={}, column={})",
            class_name.unwrap_or(Cow::Borrowed("ResolvedPos")),
            self.0.line,
            self.0.column
        )
    }
}

#[pymethods]
impl PyResolvedPos {
    #[new]
    fn py_new(line: usize, column: usize) -> Self {
        ResolvedPos { line, column }.into()
    }

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname()?.to_string();
        let me = slf.borrow();
        Ok(me.repr(Some(Cow::Owned(class_name))))
    }

    #[getter]
    fn line(&self) -> usize {
        self.0.line
    }

    #[getter]
    fn column(&self) -> usize {
        self.0.column
    }
}

#[pyclass(module = "xingque", name = "ResolvedSpan", eq, hash, frozen)]
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) struct PyResolvedSpan(ResolvedSpan);

impl From<ResolvedSpan> for PyResolvedSpan {
    fn from(value: ResolvedSpan) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyResolvedSpan {
    #[new]
    fn py_new(begin: &PyResolvedPos, end: &PyResolvedPos) -> Self {
        ResolvedSpan {
            begin: begin.0,
            end: end.0,
        }
        .into()
    }

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname()?;
        let me = slf.borrow();
        Ok(format!(
            "{}(begin={}, end={})",
            class_name,
            me.begin().repr(None),
            me.end().repr(None)
        ))
    }

    #[getter]
    fn begin(&self) -> PyResolvedPos {
        self.0.begin.into()
    }

    #[getter]
    fn end(&self) -> PyResolvedPos {
        self.0.end.into()
    }

    fn __contains__(&self, pos: &Bound<'_, PyAny>) -> PyResult<bool> {
        // TODO: handle Tuple[int, int]
        if let Ok(pos) = pos.downcast::<PyResolvedPos>() {
            Ok(self.0.contains(pos.borrow().0))
        } else {
            Err(PyValueError::new_err(
                "invalid operand type for ResolvedSpan.__contains__",
            ))
        }
    }

    fn contains(&self, pos: &Bound<'_, PyAny>) -> PyResult<bool> {
        self.__contains__(pos)
    }
}

#[pyclass(module = "xingque", name = "Span", frozen)]
pub(crate) struct PySpan(pub(crate) Span);

impl From<Span> for PySpan {
    fn from(value: Span) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PySpan {
    #[new]
    fn py_new(begin: PyPos, end: PyPos) -> Self {
        Self(Span::new(begin.0, end.0))
    }

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname()?;
        let me = slf.borrow();
        Ok(format!(
            "{}({}, {})",
            class_name,
            me.0.begin().get(),
            me.0.end().get()
        ))
    }

    fn __eq__(&self, other: &Bound<'_, PyAny>) -> bool {
        match other.downcast::<PySpan>() {
            Ok(other) => self.0 == other.borrow().0,
            Err(_) => false,
        }
    }

    #[getter]
    fn get_begin(&self) -> PyPos {
        PyPos(self.0.begin())
    }

    #[getter]
    fn get_end(&self) -> PyPos {
        PyPos(self.0.end())
    }

    fn merge(&self, other: &Self) -> Self {
        Self(self.0.merge(other.0))
    }

    fn merge_all(&self) -> Self {
        // TODO: accept an Iterable
        todo!();
    }

    fn end_span(&self) -> Self {
        Self(self.0.end_span())
    }

    fn __contains__(&self, pos: &Bound<'_, PyAny>) -> PyResult<bool> {
        if let Ok(pos) = pos.downcast::<PyPos>() {
            Ok(self.0.contains(pos.borrow().0))
        } else if let Ok(pos) = pos.extract::<u32>() {
            Ok(self.0.contains(Pos::new(pos)))
        } else {
            Err(PyValueError::new_err(
                "invalid operand type for Span.__contains__",
            ))
        }
    }

    fn contains(&self, pos: &Bound<'_, PyAny>) -> PyResult<bool> {
        self.__contains__(pos)
    }
}

#[pyclass(module = "xingque", name = "CodeMap", frozen)]
pub(crate) struct PyCodeMap(CodeMap);

impl From<CodeMap> for PyCodeMap {
    fn from(value: CodeMap) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyCodeMap {
    #[new]
    fn py_new(filename: String, source: String) -> Self {
        Self(CodeMap::new(filename, source))
    }

    #[staticmethod]
    fn empty_static() -> Self {
        todo!();
    }

    // TODO: is it necessary to wrap id()?

    fn full_span(&self) -> PySpan {
        self.0.full_span().into()
    }

    fn file_span(&self, span: &PySpan) -> PyFileSpan {
        self.0.file_span(span.0).into()
    }

    #[getter]
    fn filename(&self) -> &str {
        self.0.filename()
    }

    fn byte_at(&self, pos: &PyPos) -> u8 {
        self.0.byte_at(pos.0)
    }

    fn find_line(&self, pos: &PyPos) -> usize {
        self.0.find_line(pos.0)
    }

    #[getter]
    fn source(&self) -> &str {
        self.0.source()
    }

    fn source_span(&self, span: &PySpan) -> &str {
        self.0.source_span(span.0)
    }

    fn line_span(&self, line: usize) -> PySpan {
        PySpan(self.0.line_span(line))
    }

    fn line_span_opt(&self, line: usize) -> Option<PySpan> {
        self.0.line_span_opt(line).map(PySpan)
    }

    fn resolve_span(&self, span: &PySpan) -> PyResolvedSpan {
        self.0.resolve_span(span.0).into()
    }

    fn source_line(&self, line: usize) -> &str {
        self.0.source_line(line)
    }

    fn source_line_at_pos(&self, pos: &PyPos) -> &str {
        self.0.source_line_at_pos(pos.0)
    }
}

#[pyclass(module = "xingque", name = "FileSpan", frozen)]
#[derive(Clone)]
pub(crate) struct PyFileSpan(FileSpan);

impl From<FileSpan> for PyFileSpan {
    fn from(value: FileSpan) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyFileSpan {
    #[new]
    fn py_new(filename: String, source: String) -> Self {
        FileSpan::new(filename, source).into()
    }

    #[getter]
    fn file(&self) -> PyCodeMap {
        self.0.file.clone().into()
    }

    #[getter]
    fn span(&self) -> PySpan {
        self.0.span.into()
    }

    #[getter]
    fn filename(&self) -> &str {
        self.0.filename()
    }

    #[getter]
    fn source_span(&self) -> &str {
        self.0.source_span()
    }

    fn resolve_span(&self) -> PyResolvedSpan {
        self.0.resolve_span().into()
    }

    fn resolve(&self) -> PyResolvedFileSpan {
        self.0.resolve().into()
    }
}

#[pyclass(module = "xingque", name = "ResolvedFileLine", eq, hash, frozen)]
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) struct PyResolvedFileLine(ResolvedFileLine);

impl From<ResolvedFileLine> for PyResolvedFileLine {
    fn from(value: ResolvedFileLine) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyResolvedFileLine {
    #[new]
    fn py_new(file: String, line: usize) -> Self {
        ResolvedFileLine { file, line }.into()
    }

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname()?;
        let me = slf.borrow();
        Ok(format!(
            "{}(file={:?}, line={})",
            class_name, me.0.file, me.0.line,
        ))
    }

    #[getter]
    fn get_file(&self) -> &str {
        &self.0.file
    }

    #[getter]
    fn get_line(&self) -> usize {
        self.0.line
    }
}

#[pyclass(module = "xingque", name = "ResolvedFileSpan", eq, hash, frozen)]
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) struct PyResolvedFileSpan(ResolvedFileSpan);

impl From<ResolvedFileSpan> for PyResolvedFileSpan {
    fn from(value: ResolvedFileSpan) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyResolvedFileSpan {
    #[new]
    fn py_new(file: String, span: &PyResolvedSpan) -> Self {
        ResolvedFileSpan { file, span: span.0 }.into()
    }

    fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname()?;
        let me = slf.borrow();
        Ok(format!(
            "{}(file={:?}, span={})",
            class_name, me.0.file, me.0.span,
        ))
    }

    #[getter]
    fn get_file(&self) -> &str {
        &self.0.file
    }

    #[getter]
    fn get_span(&self) -> PyResolvedSpan {
        self.0.span.into()
    }

    fn begin_file_line(&self) -> PyResolvedFileLine {
        self.0.begin_file_line().into()
    }
}
