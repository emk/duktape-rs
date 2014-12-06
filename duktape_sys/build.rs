extern crate gcc;

use std::default::Default;

fn main() {
    gcc::compile_library("libduktape.a", &gcc::Config {
        include_directories: vec!(Path::new("duktape/src")),
        .. Default::default()
    }, &["duktape/src/duktape.c"]);
}
