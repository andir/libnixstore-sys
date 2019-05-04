extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;
use std::io::{self, Write};


fn generate_bindings(p: PathBuf) {
    let lib = pkg_config::probe_library("nixstore-c").unwrap();
    println!("lib: {:?}", lib);
    let builder = lib.include_paths.iter().filter_map(|p| {
                let mut p = p.clone();
                p.push("nixstore-c.h");
                let stderr = io::stderr();
                let mut handle = stderr.lock();
                let s = format!("path: {:?}", p);
                handle.write(&s.into_bytes());
                if p.exists() {
                    Some(p)
                } else {
                    None
                }
    }).fold(bindgen::Builder::default(), |b, p| b.header(p.to_string_lossy()));

    let bindings = builder
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(p)
        .expect("Couldn't write bindings!");
}

fn main() {

    println!("cargo:rustc-link-lib=nixstore-c");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("bindings.rs");


    if env::var("GENERATE_BINDINGS").is_ok() {
        generate_bindings(out_path);
    } else {
        std::fs::copy("bindings/bindgen_bindings.rs", out_path).expect("Failed to copy");
    }
}
