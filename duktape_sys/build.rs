extern crate gcc;

use std::env;

fn main() {
    gcc::Config::new()
                .include("duktape/src")
                .file("duktape/src/duktape.c")
                .compile("libduktape.a");

    println!("cargo:rustc-link-search=native={}", env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=duktape");
}
