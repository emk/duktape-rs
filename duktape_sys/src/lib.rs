//! EXPERIMENTAL: Low-level, unsafe wrapper arround the duktape API.
//! 
//! Note that some of this code is generated based on the specific
//! compiler, operating system and processor, so various details may change
//! depending on the system you're targeting.  Do not assume that constants
//! or integer sizes are the same everywhere!
//!
//! We do not yet provide replacements for duktape function macros, but
//! pull requests are very welcome.

#![feature(libc)]

#![allow(non_camel_case_types)]

extern crate libc;

pub use generated::*;
pub use bindings::*;
pub use glue::*;

mod generated;
mod bindings;
mod glue;

#[test]
fn test_eval() {
    use std::ptr::null_mut;
    unsafe {
        // Create a heap.
        let ctx = duk_create_heap(None, None, None, null_mut(), None);

        // Run a short code snippet.
        let code = "2+3";
        let filename = "input";
        duk_push_lstring(ctx, filename.as_ptr() as *const i8,
                         filename.len() as duk_size_t);
        let result = duk_eval_raw(ctx, code.as_ptr() as *const i8,
                                  code.len() as duk_size_t,
                                  DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE |
                                  DUK_COMPILE_SAFE);
        assert_eq!(DUK_EXEC_SUCCESS, result);

        // Get the result and make sure it's correct.
        assert_eq!(1, duk_is_number(ctx, -1));
        assert_eq!(5.0, duk_get_number(ctx, -1));
        duk_pop(ctx);

        duk_destroy_heap(ctx);
    }
}
