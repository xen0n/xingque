# 星雀 `xingque` :sparkles::bird:

Yet another Python binding to [`starlark-rust`][starlark-rust], exposing the
[Starlark] language to your Python projects. The current version wraps
`starlark-rust` version 0.12.x.

The project's name is a [calque] of "Starlark" into Chinese. It is
pronounced *xīng què* (in Standard Pinyin) or *Hsing-ch'üeh* (in Wade-Giles).

<details>
<summary>The reason behind the curious name</summary>

I had to come up with another name for the project after discovering
[an identically named project][starlark-pyo3] after I first renamed the
project `starlark-pyo3` from `python-starlark-rs`, and that the probably
next-best alternative `pystarlark` was also taken long ago. Fortunately
though, the Chinese name is shorter to type, even shorter than "starlark"
itself...

</details>

[calque]: https://en.wikipedia.org/wiki/Calque
[starlark-go]: https://github.com/caketop/python-starlark-go
[starlark-pyo3]: https://github.com/inducer/starlark-pyo3
[starlark-rust]: https://github.com/facebook/starlark-rust
[Starlark]: https://github.com/bazelbuild/starlark

**NOTE: this project still has rough corners, do not use in production yet without due care. Expect breaking changes to the API before a 1.0 version.**

## Features

A fair amount of `starlark-rust` API has been wrapped so far. You can see the
[smoke test cases](./tests/test_smoke.py) for some examples on how to integrate
this package.

This project as compared to other known bindings:

|Feature|:sparkles::bird:|[`starlark-pyo3`][starlark-pyo3]|[`starlark-go`][starlark-go]|
|---|---|---|---|
|License|Apache-2.0|Apache-2.0|Apache-2.0|
|`py.typed`|:white_check_mark:|:x:|:white_check_mark:|
|Binding framework|PyO3|PyO3|cgo|
|[ABI3] compatibility|:white_check_mark: any Python &ge; 3.8|:x:|:x:|
|Bundled :sparkles:|Rust, 0.12.x|Rust, 0.10.x|Go, circa March 2023|
|Data marshalling :snake::arrow_right::sparkles:|:zap: native FFI|:package: via Python `json`|:zap: native FFI|
|Data marshalling :sparkles::arrow_right::snake:|:zap: native FFI|:package: via Python `json`|:zap: native FFI|
|Putting opaque :snake: values into :sparkles:|:white_check_mark:|:x:|:boom: crashes|
|Getting opaque :sparkles: values into :snake:|:white_check_mark:|:x:|:x:|
|Magic method proxying for opaque :snake: values|:white_check_mark: somewhat complete|:x:|:x:|
|Magic method proxying for opaque :sparkles: values|:wrench: WIP|:x:|:x:|
|Invoking :snake: callables from :sparkles:|:white_check_mark:|:x:|:x:|
|Invoking :sparkles: callables from :snake:|:white_check_mark:|:x:|:x:|
|Linting|:calendar: planned|:white_check_mark:|:x:|
|LSP integration|:calendar: planned|:x:|:x:|
|Profiling & code coverage|:calendar: planned|:x:|:x:|
|Structured :sparkles: documentation|:calendar: planned|:x:|:x:|

[ABI3]: https://docs.python.org/3/c-api/stable.html#stable-abi

### Objects across language boundary

Two-way data marshalling is done natively if a type is available both in Python
and Starlark. For complex and/or user-defined types such as classes, opaque
wrappers at both sides are available to allow some flexibility -- after all,
people would expect some degree of interoperability between Python and Starlark,
because Starlark is (arguably) seen by many as a dialect of Python.
Opaque Python values in Starlark all have the `pyobject` type; while opaque
Starlark values are `starlark.FrozenValue` and `starlark.Value` in Python.

Identity i.e. uniqueness for the underlying concrete objects is NOT preserved
for objects across the language boundary: for example, each time you `get` a
plain-old-data value from a `FrozenModule` a new Python object would be created.
This should not cause problems at the Starlark side, as identity comparison (the
`is` operator) is [not supported in Starlark][no-is-in-starlark] anyway, but you
should take care at the Python side. Opaque Python values in Starlark context
maintain their identities when being read back from Python though, because they
are in fact just references to the original `object` being `set`.

[no-is-in-starlark]: https://github.com/bazelbuild/starlark/blob/c8d88c388698b0ee49bc74737f56236af64da1b5/design.md#no-is-operator

`xingque` proxies an opaque Python value's most magic methods into Starlark.
This means you can pass your Python objects and callables into Starlark, and use
them largely as if the runtime is still Python.

Due to missing API in PyO3, Python to Starlark proxying of some operators are
not supported right now. These are:

* unary plus: `+x` i.e. `__pos__`
* unary minus: `-x` i.e. `__neg__`
* unary invert: `~x` i.e. `__invert__`
* absolute value: `abs(x)` i.e. `__abs__`
* floor division: `x // y` i.e. `__floordiv__`
* modulus: `x % y` i.e. `__pow__`

There are other features that are not implemented right now, but I have plans
to support in a future version. These are:

* slicing i.e. sequence-like usage
* `__{get,set,del}item__` i.e. mapping-like usage
* iterator protocol

### Memory safety

There is no enforced ownership tracking in Python, unlike Rust, so exceptions
will be thrown if one tries to use an already consumed object, for example an
`AstModule` already evaluated by an `Evaluator`.

The `starlark::values::Value` type does not allow tracking its originating heap
in the public API (at least during my investigation while implementing this
library), so beware: **Python interpreter crashes can occur** if a
`starlark.Value`'s originating heap is GC-ed but a reference to such a `Value`
is kept elsewhere and later used. More design is needed for fixing the problem;
one should expect breaking changes to the API in a future version of this library.
Meanwhile, use frozen values and modules whenever appropriate; more determinism
can never hurt.

## License

Copyright &copy; 2024 WANG Xuerui. All rights reserved.

`xingque` is licensed under the [Apache 2.0 license](./LICENSE.Apache-2.0).
