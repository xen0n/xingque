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


def test_load_stmt():
    """The original starlark-rust example "Enable the `load` statement" ported
    to Python."""

    def get_source(file: str) -> str:
        match file:
            case "a.star":
                return "a = 7"
            case "b.star":
                return "b = 6"
            case _:
                return """
load('a.star', 'a')
load('b.star', 'b')
ab = a * b
"""

    def get_module(file: str) -> xingque.FrozenModule:
        ast = xingque.AstModule.parse(file, get_source(file), xingque.Dialect.STANDARD)
        modules = {}
        for load in ast.loads:
            modules[load.module_id] = get_module(load.module_id)
        loader = xingque.DictFileLoader(modules)

        globals = xingque.Globals.standard()
        module = xingque.Module()
        eval = xingque.Evaluator(module)
        eval.set_loader(loader)
        eval.eval_module(ast, globals)
        return module.freeze()

    ab = get_module("ab.star")
    assert ab.get("ab") == 42
