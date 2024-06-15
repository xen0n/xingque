use std::collections::HashMap;

use pyo3::{prelude::*, types::PyTuple};
use starlark::values::Heap;

/// Information about the data stored on a heap.
#[pyclass(module = "starlark_pyo3", name = "HeapSummary")]
pub(crate) struct PyHeapSummary(HashMap<String, (usize, usize)>);

impl From<HashMap<String, (usize, usize)>> for PyHeapSummary {
    fn from(value: HashMap<String, (usize, usize)>) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyHeapSummary {
    /// (Count, total size) by type.
    fn summary(slf: PyRef<'_, Self>) -> HashMap<String, Bound<'_, PyTuple>> {
        let mut x = HashMap::new();
        for (k, v) in &slf.0 {
            let v = vec![v.0, v.1];
            x.insert(k.clone(), PyTuple::new_bound(slf.py(), v));
        }
        x
    }

    /// Total number of bytes allocated.
    #[getter]
    fn total_allocated_bytes(&self) -> usize {
        self.0.values().map(|(_count, bytes)| bytes).sum()
    }
}

/// A heap on which `Value`s can be allocated.
#[pyclass(module = "starlark_pyo3", name = "Heap")]
pub(crate) struct PyHeap(Heap);

impl From<Heap> for PyHeap {
    fn from(value: Heap) -> Self {
        Self(value)
    }
}

#[pymethods]
impl PyHeap {
    /// Create a new `Heap`.
    #[new]
    fn py_new() -> Self {
        Heap::new().into()
    }

    /// Number of bytes allocated on this heap, not including any memory
    /// allocated outside of the starlark heap.
    #[getter]
    fn allocated_bytes(&self) -> usize {
        self.0.allocated_bytes()
    }

    /// Peak memory allocated to this heap, even if the value is now lower
    /// as a result of a subsequent garbage collection.
    #[getter]
    fn peak_allocated_bytes(&self) -> usize {
        self.0.peak_allocated_bytes()
    }

    /// Number of bytes allocated by the heap but not yet filled.
    #[getter]
    fn available_bytes(&self) -> usize {
        self.0.available_bytes()
    }

    /// Obtain a summary of how much memory is currently allocated by this heap.
    fn allocated_summary(&self) -> PyHeapSummary {
        self.0.allocated_summary().summary().into()
    }
}
