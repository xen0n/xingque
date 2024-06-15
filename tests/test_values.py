import starlark_pyo3


def test_empty_heap():
    h = starlark_pyo3.Heap()
    assert h.allocated_bytes == 0
    assert h.peak_allocated_bytes == 0
    assert h.available_bytes == 0

    s = h.allocated_summary()
    assert s.summary() == {}
    assert s.total_allocated_bytes == 0
