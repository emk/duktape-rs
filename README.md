[![Build Status](https://travis-ci.org/emk/duktape-rs.svg)](https://travis-ci.org/emk/duktape-rs)

[Documentation][apidoc].

[apidoc]: http://www.rust-ci.org/emk/duktape-rs/doc/duktape/

WORK IN PROGRESS.

A Rust wrapper for [Duktape](http://duktape.org/).  Things to do before
this is minimally useful:

- [x] Handle non-UTF-8 strings.
- [x] Call JavaScript functions by name.
- [x] Define functions.
  - [x] Call specified Rust functions from JavaScript.
  - [x] Return errors from Rust to JavaScript.
- [ ] Convert to use `Encodable`/`Decodable` everywhere.
  - [x] Convert parameters to use `Encodable`.
  - [ ] Replace `Value` with `serialize::Json`.
  - [ ] Convert return values to use `Decodable`.
- [ ] Add nice macros.
  - [ ] Provide macro for calling functions.
  - [ ] Provide macro for defining functions.

