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
        assert 'map' in list_of_names
        assert 'filter' not in list_of_names
        assert 'debug' in list_of_names
        assert 'pprint' in list_of_names
