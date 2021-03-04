// extern crate bindgen
extern crate cxx_build;
// use std::env;

// const FILES: &[&str] = &["src/cxx_aoflagger.cc", "include/cxx_aoflagger.h"];

fn main() {
    // pkg_config::Config::new()
    //     .print_system_libs(true);

    // Tell cargo to tell rustc to link the aoflagger
    // shared library.
    // println!("cargo:rustc-link-lib=static-nobundle=stdc++");
    // println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=aoflagger");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // for file in FILES {
    //     println!("cargo:rerun-if-changed={}", file);
    // }

    // let mut builder = cc::Build::new();

    // builder
    //     .cpp(true)
    //     .warnings(true)
    //     .flag_if_supported("-std=c++11")
    //     .include("src")
    //     .include("/usr/local/include")
    //     .files(FILES)
    //     .compile("aoflagger_glue.a");

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .flag_if_supported("-Wno-nonportable-include-path")
        .include("include")
        .file("src/cxx_aoflagger.cc")
        .compile("cxx_aoflagger.a");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/cxx_aoflagger.cc");
    println!("cargo:rerun-if-changed=include/cxx_aoflagger.h");

    // let out_dir = env::var("OUT_DIR").expect("No $OUT_DIR set");

}
