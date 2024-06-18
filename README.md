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

**NOTE: this project still has rough corners, do not use in production yet without due care.**

## Features

A fair amount of `starlark-rust` API has been wrapped so far. You can see the
[smoke test cases](./tests/test_smoke.py) for some examples on how to integrate
this package.

This project as compared to other known bindings:

|Feature|:sparkles::bird:|[`starlark-pyo3`][starlark-pyo3]|[`starlark-go`][starlark-go]|
|---|---|---|---|
|License|:white_check_mark: Apache-2.0|:white_check_mark: Apache-2.0|:white_check_mark: Apache-2.0|
|Bundled Starlark implementation|Rust, 0.12.x|Rust, 0.10.x|Go, 2023|
|Data thunking|:zap: native FFI|:snail: via JSON|:zap: native FFI|
|Opaque value thunking|:white_check_mark: somewhat complete|:x:|:x:|
|Invoking Starlark from Python|:white_check_mark:|:x:|:x:|
|Invoking Python from Starlark|:white_check_mark:|:x:|:x:|

## License

Copyright &copy; 2024 WANG Xuerui. All rights reserved.

`xingque` is licensed under the [Apache 2.0 license](./LICENSE.Apache-2.0).
