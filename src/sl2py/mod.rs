use num_bigint::BigInt;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use starlark::values::dict::{DictRef, FrozenDictRef};
use starlark::values::float::StarlarkFloat;
use starlark::values::function::NativeFunction;
use starlark::values::list::ListRef;
use starlark::values::tuple::{FrozenTupleRef, TupleRef};
use starlark::values::{FrozenValue, UnpackValue, Value, ValueLike};

use crate::py2sl::SlPyObject;
use crate::values::{PyFrozenValue, PyValue};

mod native_function;
use native_function::PySlNativeFunction;

pub(crate) fn py_from_sl_frozen_value(py: Python<'_>, sl: FrozenValue) -> PyResult<PyObject> {
    if sl.is_none() {
        Ok(py.None())
    } else if let Some(x) = sl.unpack_bool() {
        Ok(x.to_object(py))
    } else if let Some(x) = sl.unpack_i32() {
        Ok(x.to_object(py))
    } else if let Some(x) = BigInt::unpack_value(sl.to_value()) {
        Ok(x.to_object(py))
    } else if let Some(x) = sl.downcast_frozen_ref::<StarlarkFloat>() {
        Ok(x.0.to_object(py))
    } else if let Some(x) = sl.to_value().unpack_str() {
        Ok(x.to_object(py))
    } else if let Some(x) = FrozenTupleRef::from_frozen_value(sl) {
        let mut elements = Vec::new();
        for elem in x.content().into_iter() {
            elements.push(py_from_sl_frozen_value(py, *elem)?);
        }
        Ok(PyTuple::new_bound(py, elements).as_any().clone().unbind())
    } else if let Some(x) = ListRef::from_frozen_value(sl) {
        let mut elements = Vec::new();
        for elem in x.content().into_iter() {
            elements.push(py_from_sl_value(py, *elem)?);
        }
        Ok(PyList::new_bound(py, elements).as_any().clone().unbind())
    } else if let Some(x) = FrozenDictRef::from_frozen_value(sl) {
        let result = PyDict::new_bound(py);
        for (k, v) in x.iter() {
            let k = py_from_sl_frozen_value(py, k)?;
            let v = py_from_sl_frozen_value(py, v)?;
            result.set_item(k, v)?;
        }
        Ok(result.as_any().clone().unbind())
    } else if let Some(x) = sl.downcast_frozen_ref::<NativeFunction>() {
        PySlNativeFunction::new_py_any(py, x.as_ref())
    } else if let Some(x) = sl.downcast_frozen_ref::<SlPyObject>() {
        Ok(x.0.clone_ref(py))
    } else {
        Ok(Py::new(py, PyFrozenValue::from(sl))?.into_any())
    }
}

pub(crate) fn py_from_sl_value(py: Python<'_>, sl: Value<'_>) -> PyResult<PyObject> {
    if sl.is_none() {
        Ok(py.None())
    } else if let Some(x) = sl.unpack_bool() {
        Ok(x.to_object(py))
    } else if let Some(x) = sl.unpack_i32() {
        Ok(x.to_object(py))
    } else if let Some(x) = BigInt::unpack_value(sl) {
        Ok(x.to_object(py))
    } else if let Some(x) = sl.downcast_ref::<StarlarkFloat>() {
        Ok(x.0.to_object(py))
    } else if let Some(x) = sl.unpack_str() {
        Ok(x.to_object(py))
    } else if let Some(x) = TupleRef::from_value(sl) {
        let mut elements = Vec::new();
        for elem in x.content().into_iter() {
            elements.push(py_from_sl_value(py, *elem)?);
        }
        Ok(PyTuple::new_bound(py, elements).as_any().clone().unbind())
    } else if let Some(x) = ListRef::from_value(sl) {
        let mut elements = Vec::new();
        for elem in x.content().into_iter() {
            elements.push(py_from_sl_value(py, *elem)?);
        }
        Ok(PyList::new_bound(py, elements).as_any().clone().unbind())
    } else if let Some(x) = DictRef::from_value(sl) {
        let result = PyDict::new_bound(py);
        for (k, v) in x.iter() {
            let k = py_from_sl_value(py, k)?;
            let v = py_from_sl_value(py, v)?;
            result.set_item(k, v)?;
        }
        Ok(result.as_any().clone().unbind())
    } else if let Some(x) = sl.downcast_ref::<NativeFunction>() {
        PySlNativeFunction::new_py_any(py, x)
    } else if let Some(x) = sl.downcast_ref::<SlPyObject>() {
        Ok(x.0.clone_ref(py))
    } else {
        Ok(Py::new(py, PyValue::from(sl))?.into_any())
    }
}

pub(crate) fn py_from_sl_value_option(
    py: Python<'_>,
    sl: Option<Value<'_>>,
) -> PyResult<Option<PyObject>> {
    sl.map(|v| py_from_sl_value(py, v)).transpose()
}
