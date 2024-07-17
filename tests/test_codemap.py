import pytest
import xingque


def test_pos():
    a = xingque.Pos(233)
    b = xingque.Pos(233)
    c = xingque.Pos(234)
    assert repr(a) == "Pos(233)"
    assert a.get() == 233
    assert int(a) == 233
    assert a == 233
    assert a == b
    assert a + 1 == c
    b += 1
    assert b == c

    assert a != 233.0
    assert a != "233"
    assert a is not None
    assert a != (lambda x: x + 1)


def test_resolved_pos():
    a = xingque.ResolvedPos(12, 34)
    assert repr(a) == "ResolvedPos(line=12, column=34)"
    assert a.line == 12
    assert a.column == 34

    b = xingque.ResolvedPos(12, 34)
    c = xingque.ResolvedPos(56, 78)
    assert a == b
    assert a != c
    with pytest.raises(TypeError):
        assert a != (12, 34)
    assert hash(a) == hash(b)
    assert hash(a) != hash(c)

    with pytest.raises(AttributeError, match=r"is not writable"):
        a.line = 111


def test_resolved_span():
    a = xingque.ResolvedSpan(xingque.ResolvedPos(12, 34), xingque.ResolvedPos(23, 45))
    assert (
        repr(a)
        == "ResolvedSpan(begin=ResolvedPos(line=12, column=34), end=ResolvedPos(line=23, column=45))"
    )
    assert a.begin == xingque.ResolvedPos(12, 34)
    assert a.end == xingque.ResolvedPos(23, 45)

    b = xingque.ResolvedSpan(xingque.ResolvedPos(12, 34), xingque.ResolvedPos(23, 45))
    c = xingque.ResolvedSpan(xingque.ResolvedPos(23, 45), xingque.ResolvedPos(34, 56))
    assert a == b
    assert a != c
    with pytest.raises(TypeError):
        assert a != ((12, 34), (23, 45))
    assert hash(a) == hash(b)
    assert hash(a) != hash(c)
