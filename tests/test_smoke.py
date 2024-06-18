import xingque


def test_smoke():
    text = """
def square(x):
    return x * x

a = 42
b = square(a)

b + 1 + int(bool(233))
"""

    am = xingque.AstModule.parse("test.star", text)
    g = xingque.Globals.standard()
    e = xingque.Evaluator()
    result = e.eval_module(am, g)
    assert result == 1766

    assert set(e.module.names()) == {"a", "b", "square"}
    assert e.module.get("a") == 42
    # TODO: wrap the "function" type somehow
    # sq = e.module.get('square')
    # assert e.eval_function(sq, 12) == 144
