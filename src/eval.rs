use std::collections::HashMap;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use starlark::environment::Module;
use starlark::eval::Evaluator;

use crate::codemap::PyFileSpan;
use crate::environment::{PyGlobals, PyModule};
use crate::syntax::PyAstModule;
use crate::{py2sl, sl2py};

// it seems the Evaluator contains many thread-unsafe states
#[pyclass(module = "xingque", name = "Evaluator", unsendable)]
pub(crate) struct PyEvaluator(
    Evaluator<'static, 'static>,
    // this reference is necessary for memory safety
    #[allow(dead_code)] Py<PyModule>,
);

impl PyEvaluator {
    fn new(module: Bound<'_, PyModule>) -> Self {
        let module_ref = module.clone().unbind();
        let module = &module.borrow().0;
        let module: &'static Module = unsafe { ::core::mem::transmute(module) };
        Self(Evaluator::new(module), module_ref)
    }
}

#[pymethods]
impl PyEvaluator {
    #[new]
    fn py_new(py: Python, module: Option<Bound<'_, PyModule>>) -> PyResult<Self> {
        let module =
            module.map_or_else(|| Bound::new(py, PyModule::from(Module::new())), Result::Ok)?;
        Ok(Self::new(module))
    }

    // TODO: disable_gc

    fn eval_statements(
        &mut self,
        py: Python,
        statements: &Bound<'_, PyAstModule>,
    ) -> PyResult<PyObject> {
        match self
            .0
            .eval_statements(statements.borrow_mut().take_inner()?)
        {
            Ok(sl) => sl2py::py_from_sl_value(py, sl),
            Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
        }
    }

    fn local_variables(&self, py: Python) -> PyResult<HashMap<String, PyObject>> {
        let vars = self.0.local_variables();
        let mut result = HashMap::with_capacity(vars.len());
        for (k, v) in vars.into_iter() {
            result.insert(k.to_string(), sl2py::py_from_sl_value(py, v)?);
        }
        Ok(result)
    }

    fn verbose_gc(&mut self) {
        self.0.verbose_gc()
    }

    fn enable_static_typechecking(&mut self, enable: bool) {
        self.0.enable_static_typechecking(enable)
    }

    // TODO: set_loader
    // TODO: enable_profile
    // TODO: write_profile
    // TODO: gen_profile
    // TODO: coverage

    fn enable_terminal_breakpoint_console(&mut self) {
        self.0.enable_terminal_breakpoint_console()
    }

    // TODO: call_stack
    // TODO: call_stack_top_frame

    fn call_stack_count(&self) -> usize {
        self.0.call_stack_count()
    }

    fn call_stack_top_location(&self) -> Option<PyFileSpan> {
        self.0.call_stack_top_location().map(PyFileSpan::from)
    }

    // TODO: set_print_handler
    // TODO: heap

    #[getter]
    fn module(&self) -> Py<PyModule> {
        self.1.clone()
    }

    // TODO: frozen_heap
    // TODO: set_module_variable_at_some_point (is this okay to expose?)

    fn set_max_callstack_size(&mut self, stack_size: usize) -> PyResult<()> {
        self.0.set_max_callstack_size(stack_size)?;
        Ok(())
    }

    fn eval_module(
        &mut self,
        py: Python,
        ast: &Bound<'_, PyAstModule>,
        globals: &Bound<'_, PyGlobals>,
    ) -> PyResult<PyObject> {
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
        let heap = self.0.heap();
        let to_sl = |x| py2sl::sl_value_from_py(x, heap);
        let function = to_sl(function);
        let positional: Vec<_> = args.as_slice().iter().map(to_sl).collect();
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
