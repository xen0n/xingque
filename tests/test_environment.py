import starlark_pyo3


def test_globals_empty():
    x = starlark_pyo3.Globals()
    assert not list(x.names)
    assert x.describe() == ""
    assert x.docstring is None


def test_globals_standard():
    x = starlark_pyo3.Globals.standard()
    assert len(list(x.names)) > 0
    assert len(x.describe()) > 0
    assert x.docstring is None
