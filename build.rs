extern crate bindgen;

use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    let mut config = Config::new(".");
    let target = config.build_target("n2n");
    let dst = target.build();

    let profile = config.get_profile();

    let lib_dir = if env::consts::OS == "windows" { format!("build/n2n/{}", profile) } else { "build/n2n".to_string() };

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=n2n");
    println!("cargo:rustc-flags=-L{}", dst.join(lib_dir).display());

    let mut clang_args = vec!["-DCMAKE_BUILD"];
    if env::consts::OS == "windows" {
        clang_args.push("-DWIN32");
        clang_args.push("-In2n/win32");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .header("n2n/include/n2n.h")
        .derive_default(true)
        .size_t_is_usize(env::consts::OS != "windows")
        .no_default("tagMONITORINFOEXA")
        .clang_args(clang_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
