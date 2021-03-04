extern crate bindgen;
// extern crate pkg_config;

use std::env;
// use std::path::PathBuf;
// use std::path::Path;

const FILES: &[&str] = &["src/glue.cc"];


fn main() {
    // pkg_config::Config::new()
    //     .print_system_libs(true);

    // Tell cargo to tell rustc to link the aoflagger
    // shared library.
    // println!("cargo:rustc-link-lib=static-nobundle=stdc++");
    // println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=aoflagger");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    for file in FILES {
        println!("cargo:rerun-if-changed={}", file);
    }

    let mut builder = cc::Build::new();

    builder
        .cpp(true)
        .warnings(true)
        .flag_if_supported("-std=c++11")
        .include("src")
        .include("/usr/local/include")
        .files(FILES)
        .compile("aoflagger_glue.a");
}
