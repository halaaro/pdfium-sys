// Copyright 2023 pdfium-sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(link_cplusplus)]
extern crate link_cplusplus;

mod pdfium_build;

use std::{env, fs};

use pdfium_build::{gclient, gn, ninja, path};

fn main() {
    if cfg!(feature = "dynamic_link") {
        link_dynamic();
    } else {
        link_static();
    }
    #[cfg(feature = "bindgen")]
    generate_bindings();

    if cfg!(feature = "pdfium_build") {
        #[cfg(windows)]
        eprintln!("HELP: If pdfium's build complains about missing Debugging Tools for Windows, go to Apps & Features -> Windows Software Development Kit -> Modify -> Choose \"Change\" -> Tick \"Debugging Tools for Windows\"");

        path::mkdirs(&path::gclient_build_dir());
        gclient::config();
        gclient::sync();
        gn::gen();
        ninja::compile();
        println!(
            "cargo:rustc-link-search=native={}",
            path::pdfium_lib_dir()
                .to_str()
                .expect("pdfium directory path was not UTF-8")
        );
    } else if let Some(path) = env_dir("PDFIUM_LIB_DIR") {
        println!("cargo:rustc-link-search=native={path}");
    }
}

fn link_static() {
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=pdfium");
    } else {
        println!("cargo:rustc-link-lib=static=pdfium");
    }
    if !cfg!(feature = "link_cplusplus") {
        println!("cargo:rustc-link-lib=static:-bundle=stdc++");
    }
}

fn link_dynamic() {
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=pdfium.dll");
    } else {
        println!("cargo:rustc-link-lib=dylib=pdfium");
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    eprintln!("HELP: If bindgen fails due to missing LLVM, you may want to download and install LLVM. Check the release page on GitHub: https://github.com/llvm/llvm-project/releases");

    use std::path::PathBuf;

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let include_path = if let Some(path) = env_dir("PDFIUM_INCLUDE") {
        let mut p = PathBuf::from(path);
        if p.is_relative() {
            p = std::env::current_dir().unwrap().join(p);
        }
        p
    } else {
        let mut public_include = pdfium_build::path::src_dir();
        public_include.push("public");
        public_include
    };
    builder = builder.clang_arg(format!(
        "-I{}",
        include_path
            .canonicalize()
            .expect("could not canonicalize include path")
            .to_str()
            .expect("include path was not UTF-8")
    ));

    let bindings = builder
        // Try to keep original comments for docs
        .clang_args([
            "-fretain-comments-from-system-headers",
            "-fparse-all-comments",
        ])
        .generate_comments(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the src/bindings.rs file.
    let src_path = PathBuf::from("src");
    bindings
        .write_to_file(src_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn env_dir(env_name: &str) -> Option<String> {
    if let Ok(path) = env::var(env_name) {
        match fs::metadata(&path) {
            Ok(meta) if meta.is_dir() => return Some(path),
            _ => panic!("Invalid {}: `{}`, expected valid dir.", env_name, path),
        }
    }
    None
}
