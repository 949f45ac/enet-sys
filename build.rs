extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {
    let target = env::var("TARGET").unwrap();
    let is_debug = env::var("DEBUG").unwrap() == "true";
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/enet/include/")
	.size_t_is_usize(true)
        .header("wrapper.h")
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    
    let dst = Config::new("vendor/enet")
                .build_target("enet")
                .build();

    eprintln!("LUL: {}", dst.display());

    println!("cargo:rustc-link-lib=enet");
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=ws2_32");
    }
}
