extern crate bindgen;

use std::env;
use std::fmt::format;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    let mut config = Config::new(".");
    let mut target = config.build_target("n2n");
    if env::consts::OS == "windows" {
        target = target.build_target("n2n_win32");
    }
    let dst = target.build();

    let profile = config.get_profile();

    println!("cargo:rustc-link-search=native={}", dst.display());

    println!("cargo:rustc-link-lib=static=n2n");
    if env::consts::OS == "windows" {
        println!("cargo:rustc-link-lib=static=edge_utils_win32");
        println!("cargo:rustc-link-lib=static=n2n_win32");
    }

    if env::consts::OS == "windows" {
        println!("cargo:rustc-flags=-L{}", dst.join(format!("build/n2n/{}", profile)).display());
        println!("cargo:rustc-flags=-L{}", dst.join(format!("build/n2n/win32/{}", profile)).display());
    } else {
        println!("cargo:rustc-flags=-L{}", dst.join("build/n2n").display());
    }

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
