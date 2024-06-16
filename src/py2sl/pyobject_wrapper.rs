use std::hash::Hasher;

use allocative::Allocative;
use num_bigint::BigInt;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::intern;
use pyo3::prelude::*;
use starlark::any::ProvidesStaticType;
use starlark::collections::StarlarkHasher;
use starlark::values::{
    starlark_value, AllocFrozenValue, AllocValue, Freeze, Freezer, FrozenHeap, FrozenValue, Heap,
    NoSerialize, StarlarkValue, Trace, Value,
};

use crate::sl2py::py_from_sl_value;

#[derive(Trace, NoSerialize, ProvidesStaticType, Allocative)]
pub(crate) struct SlPyObjectWrapper(#[allocative(skip)] pub(crate) PyObject);

impl From<PyObject> for SlPyObjectWrapper {
    fn from(value: PyObject) -> Self {
        Self(value)
    }
}

impl<'v> AllocValue<'v> for SlPyObjectWrapper {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

impl AllocFrozenValue for SlPyObjectWrapper {
    fn alloc_frozen_value(self, heap: &FrozenHeap) -> FrozenValue {
        heap.alloc_simple(self)
    }
}

impl ::core::fmt::Debug for SlPyObjectWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::core::fmt::Debug::fmt(&self.0, f)
    }
}

impl ::std::fmt::Display for SlPyObjectWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::fmt::Display::fmt(&self.0, f)
    }
}

impl Freeze for SlPyObjectWrapper {
    type Frozen = SlPyObjectWrapper;

    fn freeze(self, _freezer: &Freezer) -> anyhow::Result<Self::Frozen> {
        Ok(self)
    }
}

#[starlark_value(type = "pyobject")]
impl<'v> StarlarkValue<'v> for SlPyObjectWrapper {
    type Canonical = Self;

    fn to_bool(&self) -> bool {
        let result: PyResult<bool> = Python::with_gil(|py| {
            let inner = self.0.bind(py);
            let meth = intern!(py, "__bool__");
            if inner.hasattr(meth)? {
                inner.call_method0(meth)?.extract::<bool>()
            } else {
                Ok(true)
            }
        });
        result.unwrap_or(true)
    }

    fn write_hash(&self, hasher: &mut StarlarkHasher) -> starlark::Result<()> {
        let result: PyResult<()> = Python::with_gil(|py| {
            let inner = self.0.bind(py);
            let meth = intern!(py, "__hash__");
            if inner.hasattr(meth)? {
                let hash_value = inner.call_method0(meth)?;
                if let Ok(val) = hash_value.extract::<u64>() {
                    hasher.write_u64(val);
                    return Ok(());
                }
                if let Ok(val) = hash_value.extract::<BigInt>() {
                    for limb in val.iter_u64_digits() {
                        hasher.write_u64(limb);
                    }
                    return Ok(());
                }
            }

            // no __hash__ or its return value isn't an int
            Err(PyNotImplementedError::new_err(
                "the wrapped object seems un-hashable",
            ))
        });

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(starlark::Error::new(starlark::ErrorKind::Value(e.into()))),
        }
    }

    fn equals(&self, other: Value<'v>) -> starlark::Result<bool> {
        let result: PyResult<bool> = Python::with_gil(|py| {
            let inner = self.0.bind(py);
            let other = py_from_sl_value(py, other)?;
            inner.eq(other)
        });

        result.map_err(|e| starlark::Error::new(starlark::ErrorKind::Value(e.into())))
    }
}
