import xingque


def test_globals_empty():
    x = xingque.Globals()
    assert not list(x.names())
    assert x.describe() == ""
    assert x.docstring is None


def test_globals_standard():
    x = xingque.Globals.standard()
    assert len(list(x.names())) > 0
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
        set_of_names = set(x.names())
        assert "map" in set_of_names
        assert "filter" not in set_of_names
        assert "debug" in set_of_names
        assert "pprint" in set_of_names


def test_globals_builder():
    class Opaque:
        pass

    opaque = Opaque()
    kv = {
        "foo0": None,
        "foo1": False,
        "foo2": True,
        "foo3": 233,
        "foo4": -233,
        "foo5": 0x80000000,
        "foo6": -0x100000000,
        "foo7": 1234567890123456789012345678901234567890,
        "foo8": 3.14,
        "foo9": "bar",
        "foo10": [None, 123, "quux", {1: 2, "aa": None, "l": [False, 0.1, opaque]}],
        "foo11": {"a": [123, 1 << 100], "b": False, 233: 234, None: {1: 2, 3: ""}},
        "foo12": opaque,
        "foo13": ...,
    }

    gb = xingque.GlobalsBuilder()
    for k, v in kv.items():
        gb.set(k, v)
    g = gb.build()

    set_of_names = set(g.names())
    assert len(set_of_names) == len(kv)

    for k, v in g:
        assert isinstance(k, str)
        assert k in kv
        assert v == kv[k]
        if k in {"foo12", "foo13"}:
            assert v is kv[k]


def test_module_extra_value():
    m = xingque.Module()
    assert m.extra_value is None
    v = lambda x: x + 1
    m.extra_value = v
    assert m.extra_value is v
