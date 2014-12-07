use std::borrow::Cow;
use std::ptr::null_mut;
use std::slice::from_raw_buf;
use std::str::from_utf8;
use ffi::*;
use types::*;

/// Convert a duktape-format string into a Rust `String`.
unsafe fn from_lstring(data: *const i8, len: duk_size_t) ->
    DuktapeResult<String>
{
    match from_utf8(from_raw_buf(&(data as *const u8), len as uint)) {
        None => Err(DuktapeError::from_str("can't convert string")),
        Some(ref str) => Ok(str.to_string())
    }
}

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

    /// Get the specified value from our context, and convert it to a Rust
    /// type.  This is a low-level, unsafe function, and you won't normally
    /// need to call it.
    unsafe fn get(&mut self, idx: duk_idx_t) -> DuktapeResult<Value> {
        match duk_get_type(self.ptr, idx) {
            DUK_TYPE_UNDEFINED => Ok(Value::Undefined),
            DUK_TYPE_NULL => Ok(Value::Null),
            DUK_TYPE_BOOLEAN => {
                let val = duk_get_boolean(self.ptr, idx);
                Ok(Value::Bool(val != 0))
            }
            DUK_TYPE_NUMBER => {
                Ok(Value::Number(duk_get_number(self.ptr, idx)))
            }
            DUK_TYPE_STRING => {
                let mut len: duk_size_t = 0;
                let str = duk_get_lstring(self.ptr, idx, &mut len);
                Ok(Value::String(Cow::Owned(try!(from_lstring(str, len)))))
            }
            _ => panic!("Cannot convert duktape data type")
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
                self.get(-1)
            } else {
                let mut len: duk_size_t = 0;
                let str = duk_safe_to_lstring(self.ptr, -1, &mut len);
                let msg = try!(from_lstring(str, len));
                Err(DuktapeError::from_str(msg.as_slice()))
            }
        }
    }
}

impl Drop for Context {
  fn drop(&mut self) {
      unsafe { duk_destroy_heap(self.ptr); }
  }
}

#[test]
fn test_eval() {
    let mut ctx = Context::new().unwrap();
    assert_eq!(Value::Undefined, ctx.eval("undefined").unwrap());
    assert_eq!(Value::Null, ctx.eval("null").unwrap());
    assert_eq!(Value::Bool(true), ctx.eval("true").unwrap());
    assert_eq!(Value::Bool(false), ctx.eval("false").unwrap());
    assert_eq!(Value::Number(5.0), ctx.eval("2 + 3").unwrap());
    assert_eq!(Value::String(Cow::Borrowed("é")), ctx.eval("'é'").unwrap());
    //assert_eq!(Value::String(Cow::Borrowed("𓀀")), ctx.eval("'𓀀'").unwrap());
}

#[test]
fn test_eval_errors() {
    let mut ctx = Context::new().unwrap();
    assert_eq!(true, ctx.eval("3 +").is_err());
}

//let args = vec!(Value::Number(2.0), Value::Number(3.0));
//let code = "function sum(x, y) { return x+y; }"
