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
