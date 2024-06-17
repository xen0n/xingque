use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use starlark::environment::Module;
use starlark::eval::Evaluator;

use crate::codemap::PyFileSpan;
use crate::environment::{PyGlobals, PyModule};
use crate::sl2py;
use crate::syntax::PyAstModule;

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
    // TODO: eval_statements
    // TODO: local_variables

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
}
