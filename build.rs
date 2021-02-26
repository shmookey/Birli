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
        // .whitelist_type("__darwin_(ct_rune|clock|dev|id|mbstate|mode|off|pid|rune|size|ssize|suseconds|time|uid|wchar|wctrans|wctype|wint)_t")
        // .whitelist_type("__(int32|int64|uint16|uint32|uint64|mbstate)_t")
        // .whitelist_type("__sFILE")
        // .whitelist_type("__sFILEX")
        // .whitelist_type("__sbuf")
        // .whitelist_type("__siginfo")
        // .whitelist_type("__va_list_tag")
        // .whitelist_type("_Rune(Entry|CharClass|Locale|Range)")
        // .whitelist_type("_Tp")
        // .whitelist_type("(clock|clockid|dev|div|fpos|id|idtype|ldiv|lldiv|mbstate|mode|off|pid|rlim|siginfo|size|ssize|time|uid|wctrans|wctype|wint)_t")
        // .whitelist_type("rusage")
        // .whitelist_type("rlimit")
        // .whitelist_type("sigval")
        // .whitelist_type("FILE")
        // .whitelist_type("std___(any|ignore_t|libcpp_debug_function_type|libcpp_debug_info|rs_default_result_type|rs_default|shared_count|shared_weak_count|sp_mut)")
        // .whitelist_type("std_(allocator_arg_t|bad_alloc|bad_array_new_length|bad_cast|bad_exception|bad_typeid|bad_weak_ptr|basic_string_size_type|domain_error|exception_ptr|exception|float_denorm_style|float_round_style|invalid_argument|length_error|logic_error|nested_exception|new_handler|nothrow_t|out_of_range|overflow_error|piecewise_construct_t|range_error|runtime_error|string|terminate_handler|type_info|underflow_error|unexpected_handler|unique_ptr|wstring)")
        // .whitelist_type("timespec")
        // .whitelist_type("timeval")
        // .whitelist_type("tm")
        // .opaque_type("std::string")
        // .no_copy("std::basic_string_value_type")
        // .no_debug("std::basic_string_value_type")
        // .no_default("std::basic_string_value_type")
        // .whitelist_type("std::allocator")
        // .whitelist_type("std::basic_string")
        // .whitelist_function("aoflagger_AOFlagger_GetVersionString")
        // .whitelist_type("aoflagger_AOFlagger")
        // .whitelist_type("aoflagger_AOFlagger.*")
        // .whitelist_function("aoflagger_AOFlagger_GetVersion")
        // .whitelist_type("aoflagger_(AOFlagger|FlagMask|ImageSet|QualityStatistics|Strategy|TelescopeId).*")
        // .whitelist_type("aoflagger::AOFlagger")
        // .whitelist_type("aoflagger::FlagMask")
        // .whitelist_type("aoflagger::ImageSet")
        // .whitelist_type("aoflagger::TelescopeId")
        // .whitelist_type("aoflagger::.*")
        // .whitelist_type("aoflagger_.*")
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
        .blacklist_function("aoflagger.*AOFlagger.*WriteStatistics")
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
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .expect("Couldn't write bindings!");
    bindings.write_to_file("bindings.rs").expect("hi");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    //     // .clang_arg("-x")
    //     // .clang_arg("c++")
    //     .generate_comments(true)
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("wrapped_aoflagger.hpp")
    //     .clang_arg("-std=c++11")
    //     .use_core()
    //     // .whitelist_type("aoflagger::TelescopeId")
    //     .blacklist_type("std::.*")
    //     .blacklist_type("aoflagger::.*")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     // Finish the builder and generate the bindings.
    //     .generate()
    // Unwrap the Result and panic on failure.
    // .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
