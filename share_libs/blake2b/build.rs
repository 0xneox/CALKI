#![allow(deprecated)]
extern crate gcc;

fn main() {
    gcc::compile_library("libblake2b.a", &["src/blake2b.c"]);
}
