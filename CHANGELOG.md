# Changelog

## x.x.x (UNRELEASED)

* placeholder

## 0.2.0 (2024-06-25)

* Upgraded to PyO3 0.22.0.
* The Starlark `pyobject` type now supports proxying the following operators:
    * Unary positive (`+self`)
    * Unary negative (`-self`)
    * Floor division (`self // other`)
    * Modulus (`self % other`)
    * Bit-wise not (`~other`)

## 0.1.0 (2024-06-19)

* Initial tagged release
