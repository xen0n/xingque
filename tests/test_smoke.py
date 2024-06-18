import functools

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

    sq = e.module.get("square")
    assert e.eval_function(sq, 12) == 144


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


def test_magic_method_forwarding():
    @functools.total_ordering
    class Foo:
        def __init__(self) -> None:
            self.methods_called: list[str] = []

        def __bool__(self) -> bool:
            self.methods_called.append("bool")
            return False

        def __eq__(self, other: object) -> bool:
            self.methods_called.append("eq")
            return other == 123

        def __lt__(self, other: int) -> bool:
            self.methods_called.append("lt")
            return other < 123

        def __len__(self) -> int:
            self.methods_called.append("len")
            return 111222

        def __contains__(self, item: object) -> bool:
            self.methods_called.append("contains")
            return item == 123

        def __call__(self, foo: int, bar: str) -> object:
            self.methods_called.append("call")
            assert foo == 123
            assert bar == "baz"
            return {"ok": True, "test": ["aaa", "bbb", "ccc"]}

    text = """
to_bool = bool(foo)
eq = foo == 123
ne = foo != 123
le = foo <= 123
lt = foo < 123
ge = foo >= 123
gt = foo > 123

length = len(foo)
contains = 123 in foo
invoke = foo(123, bar='baz')

# TODO
#getitem = foo.test
#add = foo + 123
#sub = foo - 123
#mul = foo * 123
#div = foo / 123
#mod = foo % 123
"""

    recorder = Foo()

    am = xingque.AstModule.parse("test.star", text)
    gb = xingque.GlobalsBuilder.standard()
    gb.set("foo", recorder)
    g = gb.build()
    m = xingque.Module()
    e = xingque.Evaluator(m)
    e.eval_module(am, g)

    assert recorder.methods_called == [
        "bool",
        "eq",
        "eq",
        "eq",
        "eq",
        "eq",
        "eq",
        "len",
        "contains",
        "call",
    ]
    assert not m.get("to_bool")
    assert m.get("eq")
    assert not m.get("ne")
    assert m.get("le")
    assert not m.get("lt")
    assert m.get("ge")
    assert not m.get("gt")
    assert m.get("length") == 111222
    assert m.get("contains")
    assert m.get("invoke") == {"ok": True, "test": ["aaa", "bbb", "ccc"]}
