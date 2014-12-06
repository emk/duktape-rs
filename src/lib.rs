//! Rust interface to Duktape JavaScript interpreter.

#![experimental]
#![feature(globs)]
#![warn(missing_docs)]

extern crate libc;
extern crate "duktape_sys" as ffi;

pub use types::*;
pub use context::*;

mod types;
mod context;

#[test]
fn test_eval() {
    let mut ctx = Context::new().unwrap();
    let result = ctx.eval("2 + 3").unwrap();
    assert_eq!(Value::Number(5.0), result);
}

//let args = vec!(Value::Number(2.0), Value::Number(3.0));
//let code = "function sum(x, y) { return x+y; }"
