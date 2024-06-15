import xingque


def test_pos():
    a = xingque.Pos(233)
    b = xingque.Pos(233)
    c = xingque.Pos(234)
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
