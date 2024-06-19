use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use starlark::values::dict::AllocDict;
use starlark::values::list::AllocList;
use starlark::values::tuple::AllocTuple;
use starlark::values::{FrozenHeap, FrozenValue, Heap, Value};

mod slpyobject;
pub(crate) use slpyobject::SlPyObject;

use crate::values::{PyFrozenValue, PyValue};

pub(crate) fn sl_frozen_value_from_py(
    value: &Bound<'_, PyAny>,
    heap: &FrozenHeap,
) -> PyResult<FrozenValue> {
    if value.is_none() {
        Ok(FrozenValue::new_none())
    } else if let Ok(x) = value.extract::<bool>() {
        Ok(FrozenValue::new_bool(x))
    } else if let Ok(x) = value.extract::<i64>() {
        Ok(heap.alloc(x))
    } else if let Ok(x) = value.extract::<u64>() {
        Ok(heap.alloc(x))
    } else if let Ok(x) = value.extract::<num_bigint::BigInt>() {
        Ok(heap.alloc(x))
    } else if let Ok(x) = value.extract::<f64>() {
        Ok(heap.alloc(x))
    } else if let Ok(x) = value.extract::<String>() {
        Ok(heap.alloc(x))
    } else if let Ok(x) = value.downcast::<PyTuple>() {
        let entries = {
            let mut tmp = Vec::new();
            for elem in x.iter_borrowed() {
                tmp.push(sl_frozen_value_from_py(&elem, heap)?);
            }
            tmp
        };
        Ok(heap.alloc(AllocTuple(entries)))
    } else if let Ok(x) = value.downcast::<PyList>() {
        let entries = {
            let mut tmp = Vec::new();
            for elem in x.into_iter() {
                tmp.push(sl_frozen_value_from_py(&elem, heap)?);
            }
            tmp
        };
        Ok(heap.alloc(AllocList(entries)))
    } else if let Ok(x) = value.downcast::<PyDict>() {
        let entries = {
            let mut tmp = Vec::new();
            for (k, v) in x.into_iter() {
                tmp.push((
                    sl_frozen_value_from_py(&k, heap)?,
                    sl_frozen_value_from_py(&v, heap)?,
                ));
            }
            tmp
        };
        Ok(heap.alloc(AllocDict(entries)))
    } else if let Ok(x) = value.downcast::<PyFrozenValue>() {
        Ok(x.borrow().0)
    } else if let Ok(_) = value.downcast::<PyValue>() {
        // disallow this
        Err(PyValueError::new_err(
            "Value must be frozen before use in this context",
        ))
    } else {
        Ok(heap.alloc(SlPyObject::from(value.clone().unbind())))
    }
}

pub(crate) fn sl_value_from_py<'v>(value: &Bound<'_, PyAny>, heap: &'v Heap) -> Value<'v> {
    if value.is_none() {
        Value::new_none()
    } else if let Ok(x) = value.extract::<bool>() {
        Value::new_bool(x)
    } else if let Ok(x) = value.extract::<i64>() {
        heap.alloc(x)
    } else if let Ok(x) = value.extract::<u64>() {
        heap.alloc(x)
    } else if let Ok(x) = value.extract::<num_bigint::BigInt>() {
        heap.alloc(x)
    } else if let Ok(x) = value.extract::<f64>() {
        heap.alloc(x)
    } else if let Ok(x) = value.extract::<String>() {
        heap.alloc(x)
    } else if let Ok(x) = value.downcast::<PyTuple>() {
        let entries = x.iter_borrowed().map(|elem| sl_value_from_py(&elem, heap));
        heap.alloc(AllocTuple(entries))
    } else if let Ok(x) = value.downcast::<PyList>() {
        let entries = x.into_iter().map(|elem| sl_value_from_py(&elem, heap));
        heap.alloc(AllocList(entries))
    } else if let Ok(x) = value.downcast::<PyDict>() {
        let entries = x
            .into_iter()
            .map(|(k, v)| (sl_value_from_py(&k, heap), sl_value_from_py(&v, heap)));
        heap.alloc(AllocDict(entries))
    } else if let Ok(x) = value.downcast::<PyFrozenValue>() {
        x.borrow().0.to_value()
    } else if let Ok(x) = value.downcast::<PyValue>() {
        // XXX: This is going to cause problems when value is shared cross-heap,
        // so more design is needed to correctly track each value's belonging heap.
        unsafe { ::core::mem::transmute(x.borrow().0) }
    } else {
        heap.alloc(SlPyObject::from(value.clone().unbind()))
    }
}
