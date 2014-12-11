[![Build Status](https://travis-ci.org/emk/duktape-rs.svg)](https://travis-ci.org/emk/duktape-rs)

[Documentation][apidoc].

[apidoc]: http://www.rust-ci.org/emk/duktape-rs/doc/duktape/

WORK IN PROGRESS.

A Rust wrapper for [Duktape](http://duktape.org/).  Things to do before
this is minimally useful:

- [x] Handle non-UTF-8 strings.
- [x] Call JavaScript functions by name.
- [ ] Define functions.
  - [x] Call specified Rust functions from JavaScript.
  - [x] Return errors from Rust to JavaScript.
  - [ ] Provide a nice type-checking helper for parameters.
- [ ] Provide macro for passing parameters to function calls.
