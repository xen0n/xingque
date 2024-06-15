import starlark_pyo3


def test_pos():
    a = starlark_pyo3.Pos(233)
    b = starlark_pyo3.Pos(233)
    c = starlark_pyo3.Pos(234)
    assert a.get() == 233
    assert int(a) == 233
    assert a == 233
    assert a == b
    assert a + 1 == c
    b += 1
    assert b == c

    assert a != 233.0
    assert a != "233"
    assert a != None
    assert a != (lambda x: x + 1)
