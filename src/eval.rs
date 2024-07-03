use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use pyo3::exceptions::PyRuntimeError;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use starlark::codemap::ResolvedFileSpan;
use starlark::environment::{FrozenModule, Module};
use starlark::errors::Frame;
use starlark::eval::{CallStack, Evaluator, FileLoader, ProfileMode};
use starlark::PrintHandler;

use crate::codemap::{PyFileSpan, PyResolvedFileSpan};
use crate::environment::{PyFrozenModule, PyGlobals, PyModule};
use crate::errors::PyFrame;
use crate::syntax::PyAstModule;
use crate::{py2sl, sl2py};

#[pyclass(module = "xingque", name = "CallStack")]
pub(crate) struct PyCallStack(CallStack);

impl From<CallStack> for PyCallStack {
    fn from(value: CallStack) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyCallStack {
    #[getter]
    fn frames(&self) -> Vec<PyFrame> {
        self.0.frames.clone().into_iter().map(Frame::into).collect()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // into_frames not needed because one doesn't consume values in Python
}

// it seems the Evaluator contains many thread-unsafe states
#[pyclass(module = "xingque", name = "Evaluator", unsendable)]
pub(crate) struct PyEvaluator(
    Evaluator<'static, 'static>,
    // this reference is necessary for memory safety
    #[allow(dead_code)] Py<PyModule>,
    PyObjectFileLoader,
    PyObjectPrintHandler,
);

impl PyEvaluator {
    fn new(module: Bound<'_, PyModule>) -> PyResult<Self> {
        let module_ref = module.clone().unbind();
        let module = module.borrow();
        let module = module.inner()?;
        let module: &'static Module = unsafe { ::core::mem::transmute(module) };
        Ok(Self(
            Evaluator::new(module),
            module_ref,
            PyObjectFileLoader::default(),
            PyObjectPrintHandler::default(),
        ))
    }

    fn ensure_module_available(&self, py: Python) -> PyResult<()> {
        self.1.bind(py).borrow().inner().map(|_| ())
    }
}

#[pymethods]
impl PyEvaluator {
    #[new]
    #[pyo3(signature = (module = None))]
    fn py_new(py: Python, module: Option<Bound<'_, PyModule>>) -> PyResult<Self> {
        let module =
            module.map_or_else(|| Bound::new(py, PyModule::from(Module::new())), Result::Ok)?;
        Self::new(module)
    }

    fn disable_gc(&mut self, py: Python) -> PyResult<()> {
        self.ensure_module_available(py)?;
        self.0.disable_gc();
        Ok(())
    }

    fn eval_statements(
        &mut self,
        py: Python,
        statements: &Bound<'_, PyAstModule>,
    ) -> PyResult<PyObject> {
        self.ensure_module_available(py)?;

        match self
            .0
            .eval_statements(statements.borrow_mut().take_inner()?)
        {
            Ok(sl) => sl2py::py_from_sl_value(py, sl),
            Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
        }
    }

    fn local_variables(&self, py: Python) -> PyResult<HashMap<String, PyObject>> {
        self.ensure_module_available(py)?;

        let vars = self.0.local_variables();
        let mut result = HashMap::with_capacity(vars.len());
        for (k, v) in vars.into_iter() {
            result.insert(k.to_string(), sl2py::py_from_sl_value(py, v)?);
        }
        Ok(result)
    }

    fn verbose_gc(&mut self, py: Python) -> PyResult<()> {
        self.ensure_module_available(py)?;
        self.0.verbose_gc();
        Ok(())
    }

    fn enable_static_typechecking(&mut self, py: Python, enable: bool) -> PyResult<()> {
        self.ensure_module_available(py)?;
        self.0.enable_static_typechecking(enable);
        Ok(())
    }

    fn set_loader(&mut self, py: Python, loader: &Bound<'_, PyAny>) -> PyResult<()> {
        self.ensure_module_available(py)?;
        self.2.set(loader.clone().unbind());
        let ptr: &'_ dyn FileLoader = &self.2;
        // Safety: actually the wrapper object and the evaluator are identically
        // scoped
        let ptr: &'static dyn FileLoader = unsafe { ::core::mem::transmute(ptr) };
        self.0.set_loader(ptr);
        Ok(())
    }

    fn enable_profile(&mut self, py: Python, mode: PyProfileMode) -> PyResult<()> {
        self.ensure_module_available(py)?;
        self.0.enable_profile(&mode.into())?;
        Ok(())
    }

    // TODO: write_profile
    // TODO: gen_profile

    fn coverage(&self, py: Python) -> PyResult<HashSet<PyResolvedFileSpan>> {
        self.ensure_module_available(py)?;
        Ok(self
            .0
            .coverage()
            .map(|x| x.into_iter().map(ResolvedFileSpan::into).collect())?)
    }

    fn enable_terminal_breakpoint_console(&mut self, py: Python) -> PyResult<()> {
        self.ensure_module_available(py)?;
        self.0.enable_terminal_breakpoint_console();
        Ok(())
    }

    fn call_stack(&self, py: Python) -> PyResult<PyCallStack> {
        self.ensure_module_available(py)?;
        Ok(self.0.call_stack().into())
    }

    fn call_stack_top_frame(&self, py: Python) -> PyResult<Option<PyFrame>> {
        self.ensure_module_available(py)?;
        Ok(self.0.call_stack_top_frame().map(Frame::into))
    }

    fn call_stack_count(&self, py: Python) -> PyResult<usize> {
        self.ensure_module_available(py)?;
        Ok(self.0.call_stack_count())
    }

    fn call_stack_top_location(&self, py: Python) -> PyResult<Option<PyFileSpan>> {
        self.ensure_module_available(py)?;
        Ok(self.0.call_stack_top_location().map(PyFileSpan::from))
    }

    fn set_print_handler(&mut self, py: Python, handler: &Bound<'_, PyAny>) -> PyResult<()> {
        self.ensure_module_available(py)?;
        let handler: Option<PyObject> = if handler.is_none() {
            None
        } else {
            Some(handler.clone().unbind())
        };
        self.3.set(handler);

        let ptr: &'_ dyn PrintHandler = &self.3;
        // Safety: actually the wrapper object and the evaluator are identically
        // scoped
        let ptr: &'static dyn PrintHandler = unsafe { ::core::mem::transmute(ptr) };
        self.0.set_print_handler(ptr);

        Ok(())
    }

    // TODO: heap

    #[getter]
    fn module(&self, py: Python) -> PyResult<Py<PyModule>> {
        self.ensure_module_available(py)?;
        Ok(self.1.clone_ref(py))
    }

    // TODO: frozen_heap
    // TODO: set_module_variable_at_some_point (is this okay to expose?)

    fn set_max_callstack_size(&mut self, py: Python, stack_size: usize) -> PyResult<()> {
        self.ensure_module_available(py)?;
        self.0.set_max_callstack_size(stack_size)?;
        Ok(())
    }

    fn eval_module(
        &mut self,
        py: Python,
        ast: &Bound<'_, PyAstModule>,
        globals: &Bound<'_, PyGlobals>,
    ) -> PyResult<PyObject> {
        self.ensure_module_available(py)?;

        match self
            .0
            .eval_module(ast.borrow_mut().take_inner()?, &globals.borrow().0)
        {
            Ok(sl) => sl2py::py_from_sl_value(py, sl),
            Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
        }
    }

    #[pyo3(signature = (function, *args, **kwargs))]
    fn eval_function(
        &mut self,
        py: Python,
        function: &Bound<'_, PyAny>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyObject> {
        self.ensure_module_available(py)?;

        let heap = self.0.heap();
        let to_sl = |x| py2sl::sl_value_from_py(x, heap);
        let function = to_sl(function);
        let positional: Vec<_> = args
            .iter_borrowed()
            .map(|x| py2sl::sl_value_from_py(&x, heap)) // borrowck doesn't let me use to_sl, sigh
            .collect();
        let named: Vec<_> = if let Some(kwargs) = kwargs {
            let mut tmp = Vec::with_capacity(kwargs.len());
            for (k, v) in kwargs.clone().into_iter() {
                tmp.push((k.extract::<String>()?, v));
            }
            tmp
        } else {
            Vec::new()
        };
        let named: Vec<_> = named.iter().map(|(k, v)| (k.as_str(), to_sl(v))).collect();

        match self.0.eval_function(function, &positional, &named) {
            Ok(sl) => sl2py::py_from_sl_value(py, sl),
            Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
        }
    }
}

/// How to profile starlark code.
#[pyclass(
    module = "xingque",
    name = "ProfileMode",
    rename_all = "SCREAMING_SNAKE_CASE",
    frozen,
    eq,
    hash
)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) enum PyProfileMode {
    /// The heap profile mode provides information about the time spent in each function and allocations
    /// performed by each function. Enabling this mode the side effect of disabling garbage-collection.
    /// This profiling mode is the recommended one.
    HeapSummaryAllocated,
    /// Like heap summary, but information about retained memory after module is frozen.
    HeapSummaryRetained,
    /// Like heap profile, but writes output comparible with
    /// [flamegraph.pl](https://github.com/brendangregg/FlameGraph/blob/master/flamegraph.pl).
    HeapFlameAllocated,
    /// Like heap flame, but information about retained memory after module is frozen.
    HeapFlameRetained,
    /// The statement profile mode provides information about time spent in each statement.
    Statement,
    /// Code coverage.
    Coverage,
    /// The bytecode profile mode provides information about bytecode instructions.
    Bytecode,
    /// The bytecode profile mode provides information about bytecode instruction pairs.
    BytecodePairs,
    /// Provide output compatible with
    /// [flamegraph.pl](https://github.com/brendangregg/FlameGraph/blob/master/flamegraph.pl).
    TimeFlame,
    /// Profile runtime typechecking.
    Typecheck,
}

impl From<ProfileMode> for PyProfileMode {
    fn from(value: ProfileMode) -> Self {
        match value {
            ProfileMode::HeapSummaryAllocated => Self::HeapSummaryAllocated,
            ProfileMode::HeapSummaryRetained => Self::HeapSummaryRetained,
            ProfileMode::HeapFlameAllocated => Self::HeapFlameAllocated,
            ProfileMode::HeapFlameRetained => Self::HeapFlameRetained,
            ProfileMode::Statement => Self::Statement,
            ProfileMode::Coverage => Self::Coverage,
            ProfileMode::Bytecode => Self::Bytecode,
            ProfileMode::BytecodePairs => Self::BytecodePairs,
            ProfileMode::TimeFlame => Self::TimeFlame,
            ProfileMode::Typecheck => Self::Typecheck,
            // NOTE: check if variants are added after every starlark dep bump!
            _ => unreachable!(),
        }
    }
}

impl From<PyProfileMode> for ProfileMode {
    fn from(value: PyProfileMode) -> Self {
        match value {
            PyProfileMode::HeapSummaryAllocated => Self::HeapSummaryAllocated,
            PyProfileMode::HeapSummaryRetained => Self::HeapSummaryRetained,
            PyProfileMode::HeapFlameAllocated => Self::HeapFlameAllocated,
            PyProfileMode::HeapFlameRetained => Self::HeapFlameRetained,
            PyProfileMode::Statement => Self::Statement,
            PyProfileMode::Coverage => Self::Coverage,
            PyProfileMode::Bytecode => Self::Bytecode,
            PyProfileMode::BytecodePairs => Self::BytecodePairs,
            PyProfileMode::TimeFlame => Self::TimeFlame,
            PyProfileMode::Typecheck => Self::Typecheck,
        }
    }
}

// it would be good if https://github.com/PyO3/pyo3/issues/1190 is implemented
// so we could have stronger typing
// but currently duck-typing isn't bad anyway
// this is why we don't declare this as a pyclass right now
#[derive(Debug, Default)]
pub(crate) struct PyObjectFileLoader(Option<PyObject>);

impl PyObjectFileLoader {
    fn set(&mut self, obj: PyObject) {
        self.0 = Some(obj);
    }
}

impl FileLoader for PyObjectFileLoader {
    fn load(&self, path: &str) -> anyhow::Result<FrozenModule> {
        if let Some(inner) = self.0.as_ref() {
            Python::with_gil(|py| {
                // first check if it's a PyDictFileLoader and forward to its impl
                if let Ok(x) = inner.downcast_bound::<PyDictFileLoader>(py) {
                    return x.borrow().load(path);
                }

                // duck-typing
                // call the wrapped PyObject's "load" method with the path
                // and expect the return value to be exactly PyFrozenModule
                let name = intern!(py, "load");
                let args = PyTuple::new_bound(py, &[path]);
                Ok(inner
                    .call_method_bound(py, name, args, None)?
                    .extract::<PyFrozenModule>(py)?
                    .0)
            })
        } else {
            // this should never happen because we control the only place where
            // this struct could possibly get instantiated, and a PyObject is
            // guaranteed there (remember None is also a non-null PyObject)
            unreachable!()
        }
    }
}

// a PyDict is wrapped here instead of the ReturnFileLoader (so we effectively
// don't wrap ReturnFileLoader but provide equivalent functionality that's
// idiomatic in Python), because unfortunately ReturnFileLoader has a lifetime
// parameter, but luckily it's basically just a reference to a HashMap and its
// logic is trivial.
#[pyclass(module = "xingque", name = "DictFileLoader")]
pub(crate) struct PyDictFileLoader(Py<PyDict>);

#[pymethods]
impl PyDictFileLoader {
    #[new]
    fn py_new(modules: Py<PyDict>) -> Self {
        Self(modules)
    }
}

impl FileLoader for PyDictFileLoader {
    fn load(&self, path: &str) -> anyhow::Result<FrozenModule> {
        let result: anyhow::Result<_> =
            Python::with_gil(|py| match self.0.bind(py).get_item(path)? {
                Some(v) => Ok(Some(v.extract::<PyFrozenModule>()?)),
                None => Ok(None),
            });
        result?.map(|x| x.0).ok_or(anyhow!(
            "DictFileLoader does not know the module `{}`",
            path
        ))
    }
}

// it would be good if https://github.com/PyO3/pyo3/issues/1190 is implemented
// so we could have stronger typing
// but currently duck-typing isn't bad anyway
// this is why we don't declare this as a pyclass right now
#[derive(Debug, Default)]
pub(crate) struct PyObjectPrintHandler(Option<PyObject>);

impl PyObjectPrintHandler {
    fn set(&mut self, obj: Option<PyObject>) {
        self.0 = obj;
    }
}

impl PrintHandler for PyObjectPrintHandler {
    fn println(&self, text: &str) -> anyhow::Result<()> {
        if let Some(inner) = self.0.as_ref() {
            Python::with_gil(|py| {
                // duck-typing
                // call the wrapped PyObject's "println" method with the path
                // and ignore the return value
                let name = intern!(py, "println");
                let args = PyTuple::new_bound(py, &[text]);
                inner.call_method_bound(py, name, args, None)?;
                Ok(())
            })
        } else {
            // Duplicate of starlark's stdlib::extra::StderrPrintHandler (the default
            // print handler), because it is unfortunately pub(crate), but the
            // logic is trivial.
            eprintln!("{}", text);
            Ok(())
        }
    }
}
