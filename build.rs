extern crate cxx_build;

fn main() {

    // Tell cargo to tell rustc to link the aoflagger
    // shared library.
    println!("cargo:rustc-link-lib=aoflagger");

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wno-nonportable-include-path")
        .include("include")
        .file("src/cxx_aoflagger.cc")
        .compile("cxx_aoflagger.a");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/cxx_aoflagger.cc");
    println!("cargo:rerun-if-changed=include/cxx_aoflagger.h");
}
