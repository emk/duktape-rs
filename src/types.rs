use libc::types::os::arch::c95::c_double;
use std::str::CowString;
use std::error::Error;
use std::result::Result;

/// A value that can be passed to and from JavaScript.  This does not
/// include all the types that can be stored internally!
#[deriving(Show, PartialEq)]
pub enum Value<'a> {
    /// An undefined JavaScript value.
    Undefined,
    /// A JavaScript `null` value.
    Null,
    /// A JavaScript boolean value.
    Bool(bool),
    /// A JavaScript numeric value.
    Number(c_double),
    /// A JavaScript string value.
    String(CowString<'a>)
}

/// A duktape API error.
#[deriving(Show, PartialEq)]
pub struct DuktapeError {
    // These have some sort of internal structure, but the duktape
    // documentation always just converts them to strings.  So that's all
    // we'll store for now.
    message: String
}

impl DuktapeError {
    /// Create an error, specifying an error message.
    #[experimental = "Will probably go away"]
    pub fn from_str(message: &str) -> DuktapeError {
        DuktapeError{message: message.to_string()}
    }
}

impl Error for DuktapeError {
    fn description(&self) -> &str { "script error:" }
    fn detail(&self) -> Option<String> { Some(self.message.clone()) }
    fn cause(&self) -> Option<&Error> { None }
}

/// Either a return value of type `T`, or a tuktape error.
pub type DuktapeResult<T> = Result<T, DuktapeError>;
