//! Replacements for parts of the C API that can't be used directly by Rust,
//! including preprocessor macros and varargs functions.

use generated::*;
use bindings::*;

extern "C" {
    /// A wrapper around duk_push_error_object, which relies on varargs in
    /// the original API.
    pub fn duk_push_error_object_string(
        ctx: *mut duk_context, err_code: duk_errcode_t,
        filename: *const i8, line: duk_int_t,
        message: *const i8) -> duk_idx_t;
}
