import xingque


def test_globals_empty():
    x = xingque.Globals()
    assert not list(x.names)
    assert x.describe() == ""
    assert x.docstring is None


def test_globals_standard():
    x = xingque.Globals.standard()
    assert len(list(x.names)) > 0
    assert len(x.describe()) > 0
    assert x.docstring is None
