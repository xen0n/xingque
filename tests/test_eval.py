import pytest

import xingque


def test_typechecking():
    text = """
def add(a: int) -> int:
    return a + 123

b = 123
c = add(b)
d = add('boom')
"""

    am = xingque.AstModule.parse("test.star", text, xingque.Dialect.EXTENDED)
    g = xingque.Globals.standard()
    e = xingque.Evaluator()

    with pytest.raises(RuntimeError, match=r"does not match the type annotation"):
        e.eval_module(am, g)

    # the evaluation result so far is still preserved
    assert e.module.get("c") == 246


def test_static_typechecking():
    text = """
def foo() -> int:
    return "hello"
"""

    g = xingque.Globals.standard()
    e = xingque.Evaluator()

    # Without static type-checking, this would pass
    am = xingque.AstModule.parse("test.star", text, xingque.Dialect.EXTENDED)
    e.enable_static_typechecking(False)
    e.eval_module(am, g)

    # But not if enable_static_typechecking() is called
    am = xingque.AstModule.parse("test.star", text, xingque.Dialect.EXTENDED)
    e.enable_static_typechecking(True)
    with pytest.raises(RuntimeError, match=r"Expected type `int` but got `str`"):
        e.eval_module(am, g)
