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
