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


def test_globals_extended_by():
    list_of_exts = (
        xingque.LibraryExtension.MAP,
        xingque.LibraryExtension.DEBUG,
        xingque.LibraryExtension.PPRINT,
    )

    for ty in (tuple, list, set, frozenset):
        x = xingque.Globals.extended_by(ty(list_of_exts))
        list_of_names = set(x.names)
        assert "map" in list_of_names
        assert "filter" not in list_of_names
        assert "debug" in list_of_names
        assert "pprint" in list_of_names


def test_globals_builder():
    class Opaque:
        pass

    gb = xingque.GlobalsBuilder()
    gb.set("foo0", None)
    gb.set("foo1", True)
    gb.set("foo2", 233)
    gb.set("foo3", 1234567890123456789012345678901234567890)
    gb.set("foo4", 3.14)
    gb.set("foo5", "bar")
    gb.set("foo6", [None, 123, "quux", {1: 2, "aa": None}])
    gb.set("foo7", {"a": [123, 456], "b": False, 233: 234})
    gb.set("foo8", Opaque())
    g = gb.build()
    list_of_names = set(g.names)
    assert len(list_of_names) == 9
