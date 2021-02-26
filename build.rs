extern crate bindgen;
// extern crate pkg_config;

use std::env;
// use std::path::PathBuf;
use std::path::Path;

fn main() {
    // pkg_config::Config::new()
    //     .print_system_libs(true);

    // Tell cargo to tell rustc to link the aoflagger
    // shared library.
    // println!("cargo:rustc-link-lib=static-nobundle=stdc++");
    // println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=aoflagger");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapped_aoflagger.hpp");
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::builder()
        .clang_arg("-xc++")
        .clang_arg("-std=c++11")
        .header("wrapped_aoflagger.hpp")
        .whitelist_recursively(true)
        .use_core()
        .whitelist_type("aoflagger::AOFlagger")
        .whitelist_type("aoflagger::FlagMask")
        .whitelist_type("aoflagger::ImageSet")
        .whitelist_type("aoflagger::TelescopeId")
        .whitelist_type("aoflagger::QualityStatistics")
        .whitelist_type("aoflagger::Strategy")
        .opaque_type("std::unique_ptr")
        .opaque_type("std::basic_string")
        .opaque_type("std::string")
        .blacklist_function("aoflagger.*AOFlagger.*GetVersionString")
        .blacklist_function("aoflagger.*AOFlagger.*GetVersionDate")
        .blacklist_function("aoflagger.*AOFlagger.*FindStrategyFile")
        .prepend_enum_name(true)
        .generate_comments(true)
        .rustfmt_bindings(true)
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // dbg!(&bindings);

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_dir = env::var("OUT_DIR").expect("No $OUT_DIR set");
    bindings
        .write_to_file(Path::new(&out_dir).join("aoflagger_bindings.rs"))
        .expect("Couldn't write bindings!");
    bindings
        .write_to_file("bindings.rs")
        .expect("coudn't write bindings!");
}
