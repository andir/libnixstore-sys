extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=nixstore-c");

    let include_paths = pkg_config::probe_library("nixstore-c").unwrap().include_paths;
    for path in include_paths.iter() {
        println!("cargo:include={}", path.to_string_lossy());
    }

//    let builder = include_paths.iter().fold(bindgen::Builder::default(), |b, p| b.header(p.to_string_lossy() + "/nixstore-c.h"));

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
