extern crate cxx_build;
use std::mem::size_of_val;

fn main() {
    // Tell cargo to tell rustc to link the aoflagger
    // shared library.
    println!("cargo:rustc-link-lib=aoflagger");

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wno-nonportable-include-path")
        .flag_if_supported("-Wno-unused-parameter")
        .include("include")
        .file("src/cxx_aoflagger.cc")
        .compile("cxx_aoflagger");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/cxx_aoflagger.cc");
    println!("cargo:rerun-if-changed=include/cxx_aoflagger.h");

    // test memory layout

    let usize_test: usize = 0;
    println!("cargo:warning=usize_size={}", size_of_val(&usize_test));
}
