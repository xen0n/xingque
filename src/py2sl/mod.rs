use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use starlark::values::dict::AllocDict;
use starlark::values::list::AllocList;
use starlark::values::tuple::AllocTuple;
use starlark::values::{FrozenHeap, FrozenValue};

mod pyobject_wrapper;

pub(crate) fn sl_frozen_value_from_py(value: &Bound<'_, PyAny>, heap: &FrozenHeap) -> FrozenValue {
    if value.is_none() {
        return FrozenValue::new_none();
    }
    if let Ok(x) = value.extract::<bool>() {
        return FrozenValue::new_bool(x);
    }
    if let Ok(x) = value.extract::<i64>() {
        return heap.alloc(x);
    }
    if let Ok(x) = value.extract::<u64>() {
        return heap.alloc(x);
    }
    if let Ok(x) = value.extract::<f64>() {
        return heap.alloc(x);
    }
    if let Ok(x) = value.extract::<num_bigint::BigInt>() {
        return heap.alloc(x);
    }
    if let Ok(x) = value.extract::<&str>() {
        return heap.alloc(x);
    }
    if let Ok(x) = value.downcast::<PyTuple>() {
        let entries = x
            .as_slice()
            .into_iter()
            .map(|elem| sl_frozen_value_from_py(elem, heap));
        return heap.alloc(AllocTuple(entries));
    }
    if let Ok(x) = value.downcast::<PyList>() {
        let entries = x
            .into_iter()
            .map(|elem| sl_frozen_value_from_py(&elem, heap));
        return heap.alloc(AllocList(entries));
    }
    if let Ok(x) = value.downcast::<PyDict>() {
        let entries = x.into_iter().map(|(k, v)| {
            (
                sl_frozen_value_from_py(&k, heap),
                sl_frozen_value_from_py(&v, heap),
            )
        });
        return heap.alloc(AllocDict(entries));
    }

    heap.alloc(pyobject_wrapper::SlPyObjectWrapper::from(
        value.clone().unbind(),
    ))
}
