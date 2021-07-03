## firrtl-parser

**This project is incomplete**.

`firrtl-parser` is a parser for [FIRRTL](https://github.com/chipsalliance/firrtl), the [Chips Alliance](https://chipsalliance.org/)'s "Flexible Intermediate Representation for RTL". It is implemented in rust using the oh-so-wonderful [`nom`](https://crates.io/crates/nomhttps://github.com/Geal/nom) crate.

You can read the FIRRTL Spec [here](https://raw.githubusercontent.com/chipsalliance/firrtl/master/spec/spec.pdf)

### Features

This crate will expose a single function that takes a string consisting of spec-compliant FIRRTL and returns an AST. It will never, ever do anything else. Stuff like typechecking, simulation, codegen, etc are explicitly out of scope of this crate.
