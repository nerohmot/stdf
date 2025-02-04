extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let include_dir = PathBuf::from(crate_dir.clone()).join("include");

    // Generate C header
    let c_header = include_dir.join("stdf.h");
    cbindgen::generate(crate_dir.clone())
        .expect("Unable to generate C bindings")
        .write_to_file(c_header);

    // Generate C++ header
    let cpp_header = include_dir.join("stdf.hpp");
    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::Cxx)
        .generate()
        .expect("Unable to generate C++ bindings")
        .write_to_file(cpp_header);
}