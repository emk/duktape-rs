use std::ptr::null_mut;
use ffi::*;
use types::*;

/// A duktape interpreter context.  An individual context is not
/// re-entrant: You may only access it from one thread at a time.
pub struct Context {
    ptr: *mut duk_context
}

impl Context {
    /// Create a new duktape context.
    pub fn new() -> DuktapeResult<Context> {
        let ptr = unsafe {
            duk_create_heap(None, None, None, null_mut(), None)
        };
        if ptr.is_null() {
            Err(DuktapeError::from_str("Could not create heap"))
        } else {
            Ok(Context{ptr: ptr})
        }
    }

    /// Evaluate JavaScript source code and return the result.
    pub fn eval(&mut self, code: &str) -> DuktapeResult<Value> {
        self.eval_from("<eval>", code)
    }

    /// Evaluate JavaScript source code and return the result.  The
    /// `filename` parameter will be used in any error messages.
    pub fn eval_from(&mut self, filename: &str, code: &str) ->
        DuktapeResult<Value>
    {
        unsafe {
            // Push our filename parameter and evaluate our code.
            duk_push_lstring(self.ptr, filename.as_ptr() as *const i8,
                                    filename.len() as duk_size_t);
            let result =
                duk_eval_raw(self.ptr, code.as_ptr() as *const i8,
                             code.len() as duk_size_t,
                             DUK_COMPILE_EVAL |
                             DUK_COMPILE_NOSOURCE |
                             DUK_COMPILE_SAFE);

            // Convert our result to a Rust value.
            if result == DUK_EXEC_SUCCESS {
                match duk_get_type(self.ptr, -1) {
                    DUK_TYPE_NUMBER => {
                        Ok(Value::Number(duk_get_number(self.ptr, -1)))
                    }
                    _ => panic!("Cannot convert duktape data type")
                }
            } else {
                panic!("Errors not yet implemented");
            }
        }
    }
}

impl Drop for Context {
  fn drop(&mut self) {
      unsafe { duk_destroy_heap(self.ptr); }
  }
}
