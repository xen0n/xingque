use std::collections::HashMap;

use anyhow::anyhow;
use pyo3::exceptions::PyRuntimeError;
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use starlark::environment::{FrozenModule, Module};
use starlark::eval::{Evaluator, FileLoader};

use crate::codemap::PyFileSpan;
use crate::environment::{PyFrozenModule, PyGlobals, PyModule};
use crate::syntax::PyAstModule;
use crate::{py2sl, sl2py};

// it seems the Evaluator contains many thread-unsafe states
#[pyclass(module = "xingque", name = "Evaluator", unsendable)]
pub(crate) struct PyEvaluator(
    Evaluator<'static, 'static>,
    // this reference is necessary for memory safety
    #[allow(dead_code)] Py<PyModule>,
    PyObjectFileLoader,
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

    // TODO: enable_profile
    // TODO: write_profile
    // TODO: gen_profile
    // TODO: coverage

    fn enable_terminal_breakpoint_console(&mut self, py: Python) -> PyResult<()> {
        self.ensure_module_available(py)?;
        self.0.enable_terminal_breakpoint_console();
        Ok(())
    }

    // TODO: call_stack
    // TODO: call_stack_top_frame

    fn call_stack_count(&self, py: Python) -> PyResult<usize> {
        self.ensure_module_available(py)?;
        Ok(self.0.call_stack_count())
    }

    fn call_stack_top_location(&self, py: Python) -> PyResult<Option<PyFileSpan>> {
        self.ensure_module_available(py)?;
        Ok(self.0.call_stack_top_location().map(PyFileSpan::from))
    }

    // TODO: set_print_handler
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
