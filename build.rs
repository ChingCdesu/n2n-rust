extern crate bindgen;

use std::path::PathBuf;

use cmake::Config;

fn main() {
    let dst = Config::new("n2n")
        .target("n2n")
        .out_dir("n2n")
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=n2n");
    println!("cargo:rustc-flags=-Ln2n/build");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("n2n/include/n2n.h")
        .clang_args(["-DCMAKE_BUILD"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
