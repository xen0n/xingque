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
        if file == "a.star":
            return "a = 7"
        elif file == "b.star":
            return "b = 6"
        else:
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
            self.methods_called: list[str]
            self.setattr_value: dict[str, object]

            # prevent infinite recursion
            super().__setattr__("methods_called", [])
            super().__setattr__("setattr_value", {})

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

        def __getattr__(self, name: str) -> object:
            self.methods_called.append(f"getattr:{name}")
            if name == "nonexistent":
                raise AttributeError
            return f"return_{name}"

        def __setattr__(self, name: str, value: object) -> None:
            self.methods_called.append(f"setattr:{name}")
            self.setattr_value[name] = value

        def __pos__(self) -> str:
            self.methods_called.append("pos")
            return f"pos"

        def __neg__(self) -> str:
            self.methods_called.append("neg")
            return f"neg"

        def __add__(self, rhs: int) -> str:
            self.methods_called.append("add")
            return f"add:{rhs}"

        def __sub__(self, rhs: int) -> str:
            self.methods_called.append("sub")
            return f"sub:{rhs}"

        def __mul__(self, rhs: int) -> str:
            self.methods_called.append("mul")
            return f"mul:{rhs}"

        def __truediv__(self, rhs: int) -> str:
            self.methods_called.append("truediv")
            return f"truediv:{rhs}"

        def __floordiv__(self, rhs: int) -> str:
            self.methods_called.append("floordiv")
            return f"floordiv:{rhs}"

        def __mod__(self, rhs: int) -> str:
            self.methods_called.append("mod")
            return f"mod:{rhs}"

        def __lshift__(self, rhs: int) -> str:
            self.methods_called.append("lshift")
            return f"lshift:{rhs}"

        def __rshift__(self, rhs: int) -> str:
            self.methods_called.append("rshift")
            return f"rshift:{rhs}"

        def __and__(self, rhs: int) -> str:
            self.methods_called.append("and")
            return f"and:{rhs}"

        def __or__(self, rhs: int) -> str:
            self.methods_called.append("or")
            return f"or:{rhs}"

        def __xor__(self, rhs: int) -> str:
            self.methods_called.append("xor")
            return f"xor:{rhs}"

        def __invert__(self) -> str:
            self.methods_called.append("invert")
            return f"invert"

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

get_attr = foo.test_get_attr
foo.test_set_attr = any
has_attr1 = hasattr(foo, 'methods_called')
has_attr2 = hasattr(foo, 'nonexistent')
dir_attr = dir(foo)

pos = +foo
neg = -foo

add = foo + 123
sub = foo - 123
mul = foo * 123
div = foo / 123
floordiv = foo // 123
mod = foo % 123
lshift = foo << 123
rshift = foo >> 123
bitand = foo & 123
bitor = foo | 123
bitxor = foo ^ 123
bitnot = ~foo
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
        "getattr:test_get_attr",
        "getattr:test_get_attr",
        "setattr:test_set_attr",
        "getattr:nonexistent",
        "pos",
        "neg",
        "add",
        "sub",
        "mul",
        "truediv",
        "floordiv",
        "mod",
        "lshift",
        "rshift",
        "and",
        "or",
        "xor",
        "invert",
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
    assert m.get("get_attr") == "return_test_get_attr"
    assert set(recorder.setattr_value.keys()) == {
        "test_set_attr",
    }
    assert m.get("has_attr1")
    assert not m.get("has_attr2")
    assert m.get("dir_attr") == dir(recorder)
    assert m.get("pos") == "pos"
    assert m.get("neg") == "neg"
    assert m.get("add") == "add:123"
    assert m.get("sub") == "sub:123"
    assert m.get("mul") == "mul:123"
    assert m.get("div") == "truediv:123"
    assert m.get("floordiv") == "floordiv:123"
    assert m.get("mod") == "mod:123"
    assert m.get("lshift") == "lshift:123"
    assert m.get("rshift") == "rshift:123"
    assert m.get("bitand") == "and:123"
    assert m.get("bitor") == "or:123"
    assert m.get("bitxor") == "xor:123"
    assert m.get("bitnot") == "invert"
