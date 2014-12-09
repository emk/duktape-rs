//! Rust interface to [Duktape][] JavaScript interpreter.  This is still
//! a work in progress!
//!
//! [Source code](https://github.com/emk/duktape-rs).
//!
//! ```
//! use duktape::{Context,Value,DuktapeResult};
//!
//! fn add_example() -> DuktapeResult<Value<'static>> {
//!     // Create a new JavaScript interpreter.  This will be automatically
//!     // cleaned up when `ctx` goes out of scope.
//!     let mut ctx = try!(Context::new());
//!
//!     // Load some code from a string.
//!     try!(ctx.eval("function add(x, y) { return x+y; }"));
//!
//!     // Call the function we defined.
//!     ctx.call("add", &[Value::Number(2.0), Value::Number(1.0)])
//! }
//!
//! assert_eq!(Ok(Value::Number(3.0)), add_example());
//! ```
//!
//! We also have preliminary support for defining JavaScript functions
//! using Rust, but it's still too ugly to show off.
//!
//! [Duktape]: http://duktape.org/

#![experimental]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(phase)]
#![warn(missing_docs)]

#[phase(plugin, link)] extern crate log;
extern crate libc;
extern crate cesu8;
#[phase(plugin, link)] extern crate abort_on_panic;
extern crate "duktape_sys" as ffi;

pub use types::{Value, ErrorCode, DuktapeError, DuktapeResult};
pub use context::{Context, Callback};

mod types;
mod context;
