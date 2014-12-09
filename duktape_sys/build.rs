extern crate gcc;

use std::default::Default;
use std::os::{getenv, setenv};

fn main() {
    // Make sure we get a thread-safe build.  Without this, duktape refuses
    // to set DUK_USE_VARIADIC_MACROS and falls back to global variables.
    let mut cflags = getenv("CFLAGS").unwrap_or("".to_string());
    cflags.push_str(" -std=c99");
    setenv("CFLAGS", cflags);

    gcc::compile_library("libduktape.a", &gcc::Config {
        include_directories: vec!(Path::new("duktape/src")),
        .. Default::default()
    }, &["duktape/src/duktape.c", "src/glue.c"]);
}
