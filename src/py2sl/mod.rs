use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use starlark::values::dict::AllocDict;
use starlark::values::list::AllocList;
use starlark::values::tuple::AllocTuple;
use starlark::values::{FrozenHeap, FrozenValue, Heap, Value};

mod slpyobject;
pub(crate) use slpyobject::SlPyObject;

pub(crate) fn sl_frozen_value_from_py(value: &Bound<'_, PyAny>, heap: &FrozenHeap) -> FrozenValue {
    if value.is_none() {
        FrozenValue::new_none()
    } else if let Ok(x) = value.extract::<bool>() {
        FrozenValue::new_bool(x)
    } else if let Ok(x) = value.extract::<i64>() {
        heap.alloc(x)
    } else if let Ok(x) = value.extract::<u64>() {
        heap.alloc(x)
    } else if let Ok(x) = value.extract::<num_bigint::BigInt>() {
        heap.alloc(x)
    } else if let Ok(x) = value.extract::<f64>() {
        heap.alloc(x)
    } else if let Ok(x) = value.extract::<&str>() {
        heap.alloc(x)
    } else if let Ok(x) = value.downcast::<PyTuple>() {
        let entries = x
            .as_slice()
            .into_iter()
            .map(|elem| sl_frozen_value_from_py(elem, heap));
        heap.alloc(AllocTuple(entries))
    } else if let Ok(x) = value.downcast::<PyList>() {
        let entries = x
            .into_iter()
            .map(|elem| sl_frozen_value_from_py(&elem, heap));
        heap.alloc(AllocList(entries))
    } else if let Ok(x) = value.downcast::<PyDict>() {
        let entries = x.into_iter().map(|(k, v)| {
            (
                sl_frozen_value_from_py(&k, heap),
                sl_frozen_value_from_py(&v, heap),
            )
        });
        heap.alloc(AllocDict(entries))
    } else {
        heap.alloc(SlPyObject::from(value.clone().unbind()))
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
    } else if let Ok(x) = value.extract::<&str>() {
        heap.alloc(x)
    } else if let Ok(x) = value.downcast::<PyTuple>() {
        let entries = x
            .as_slice()
            .into_iter()
            .map(|elem| sl_value_from_py(elem, heap));
        heap.alloc(AllocTuple(entries))
    } else if let Ok(x) = value.downcast::<PyList>() {
        let entries = x.into_iter().map(|elem| sl_value_from_py(&elem, heap));
        heap.alloc(AllocList(entries))
    } else if let Ok(x) = value.downcast::<PyDict>() {
        let entries = x
            .into_iter()
            .map(|(k, v)| (sl_value_from_py(&k, heap), sl_value_from_py(&v, heap)));
        heap.alloc(AllocDict(entries))
    } else {
        heap.alloc(SlPyObject::from(value.clone().unbind()))
    }
}
