use std::error::Error;
use std::result::Result;
use ffi::*;

/// These are the standard error codes, which make it easy to return
/// pre-defined errors from duktape functions implemented in Rust.
#[allow(missing_docs)]
#[derive(Copy, Show, PartialEq, Eq)]
#[repr(i32)]
pub enum ErrorCode {
    Unimplemented = DUK_ERR_UNIMPLEMENTED_ERROR,
    Unsupported   = DUK_ERR_UNSUPPORTED_ERROR,
    Internal      = DUK_ERR_INTERNAL_ERROR,
    Alloc         = DUK_ERR_ALLOC_ERROR,
    Assertion     = DUK_ERR_ASSERTION_ERROR,
    Api           = DUK_ERR_API_ERROR,
    Uncaught      = DUK_ERR_UNCAUGHT_ERROR,
    Error         = DUK_ERR_ERROR,
    Eval          = DUK_ERR_EVAL_ERROR,
    Range         = DUK_ERR_RANGE_ERROR,
    Reference     = DUK_ERR_REFERENCE_ERROR,
    Syntax        = DUK_ERR_SYNTAX_ERROR,
    Type          = DUK_ERR_TYPE_ERROR,
    Uri           = DUK_ERR_URI_ERROR
}

/// A duktape API error.  The is used as both the return type of duktape of
/// functions, and also the return type of Rust functions called from
/// duktape.
#[derive(Show, PartialEq, Eq)]
pub struct DuktapeError {
    /// The error code, if a specific one is available, or
    /// `ErrorCode::Error` if we have nothing better.
    code: ErrorCode,

    /// Errors have some sort of internal structure, but the duktape
    /// documentation always just converts them to strings.  So that's all
    /// we'll store for now.
    message: Option<String>
}

impl DuktapeError {
    /// Create an error specifying just the error code.
    pub fn from_code(code: ErrorCode) -> DuktapeError {
        DuktapeError{code: code, message: None}
    }

    /// Create an error, specifying an error message.
    pub fn from_str(message: &str) -> DuktapeError {
        DuktapeError{code: ErrorCode::Error, message: Some(message.to_string())}
    }
}

/// Re-exported within the crate, but not outside.
pub fn err_code(err: &DuktapeError) -> ErrorCode { err.code }
pub fn err_message(err: &DuktapeError) -> &Option<String> { &err.message }

impl Error for DuktapeError {
    fn description(&self) -> &str { "script error:" }

    fn detail(&self) -> Option<String> {
        self.message.clone().or_else(|| {
            let msg = match self.code {
                ErrorCode::Error => "an unknown error occurred".to_string(),
                code => format!("type: {:?} code: {:?}", code, code as duk_int_t)
            };
            Some(msg)
        })
    }

    fn cause(&self) -> Option<&Error> { None }
}

/// Either a return value of type `T`, or a duktape error.
pub type DuktapeResult<T> = Result<T, DuktapeError>;
