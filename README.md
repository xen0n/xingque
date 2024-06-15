# starlark-pyo3

A Python binding to [`starlark-rs`][starlark-rs], exposing the [Starlark]
language to your Python projects. The current version wraps `starlark-rs`
version 0.12.x.

[starlark-rs]: https://github.com/facebook/starlark-rust
[Starlark]: https://github.com/bazelbuild/starlark

**NOTE: this project is still under development, do not use in production yet.**

## Work in Progress

* [ ] `starlark::analysis`
* [ ] `starlark::any`
* [ ] `starlark::assert`
* [x] `starlark::codemap` -- majority completed
    - [x] `CodeMap`
    - [ ] `CodeMapId`
    - [x] `FileSpan`
    - [ ] `FileSpanRef`
    - [ ] `NativeCodeMap`
    - [x] `Pos`
    - [x] `ResolvedFileLine`
    - [x] `ResolvedFileSpan`
    - [x] `ResolvedPos`
    - [x] `ResolvedSpan`
    - [x] `Span`
    - [ ] `Spanned`
* [ ] `starlark::coerce`
* [ ] `starlark::collections`
* [ ] `starlark::debug`
* [ ] `starlark::docs`
* [ ] `starlark::environment`
    - [ ] `FrozenModule`
    - [x] `Globals` -- partially done
    - [ ] `GlobalsBuilder`
    - [ ] `GlobalsStatic`
    - [ ] `Methods`
    - [ ] `MethodsBuilder`
    - [ ] `MethodsStatic`
    - [ ] `Module`
    - [ ] `LibraryExtension`
* [ ] `starlark::errors`
* [ ] `starlark::eval`
* [x] `starlark::syntax`
    - [x] `DialectTypes`
    - [x] `Dialect`
    - [x] `AstLoad`
    - [x] `AstModule`
* [ ] `starlark::typing`
* [ ] `starlark::values`
    - [ ] `AggregateHeapProfileInfo`
    - [ ] `Demand`
    - [ ] `Freezer`
    - [ ] `FrozenHeap`
    - [ ] `FrozenHeapRef`
    - [ ] `FrozenRef`
    - [ ] `FrozenValue`
    - [ ] `FrozenValueTyped`
    - [x] `Heap` -- partially done
    - [ ] `OwnedFrozenRef`
    - [ ] `OwnedFrozenValue`
    - [ ] `OwnedFrozenValueTyped`
    - [ ] `StarlarkIterator`
    - [ ] `StarlarkStrNRepr`
    - [ ] `Tracer`
    - [ ] `Value`
    - [ ] `ValueIdentity`
    - [ ] `ValueOf`
    - [ ] `ValueOfUnchecked`
    - [ ] `ValueTyped`
    - [ ] `ValueTypedComplex`

## License

Copyright &copy; 2024 WANG Xuerui. All rights reserved.

`starlark-pyo3` is licensed under either the
[Apache 2.0 license](./LICENSE.Apache-2.0) or the
[MIT license](./LICENSE.MIT).
