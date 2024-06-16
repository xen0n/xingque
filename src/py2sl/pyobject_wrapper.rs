use allocative::Allocative;
use pyo3::prelude::*;
use starlark::any::ProvidesStaticType;
use starlark::values::{
    starlark_value, AllocFrozenValue, AllocValue, Freeze, Freezer, FrozenHeap,
    FrozenValue, Heap, NoSerialize, StarlarkValue, Trace, Value,
};

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
}
