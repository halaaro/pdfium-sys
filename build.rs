// Copyright 2023 pdfium-sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod pdfium_build;
#[cfg(windows)]
mod windows;

use std::{env, fs};

use pdfium_build::path::pdfium_lib_dir;

use crate::pdfium_build::{depot_tools, gclient, gn, ninja, path};

fn main() {
    if cfg!(feature = "pdfium_build") {
        depot_tools::clone();

        #[cfg(windows)]
        windows::init();

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

    if cfg!(feature = "dynamic_link") {
        link_dynamic();
    } else {
        link_static();
        link_custom_libcxx();
    }

    #[cfg(feature = "bindgen")]
    generate_bindings();
}

fn link_static() {
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=pdfium");
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=msvcprt");
    } else {
        println!("cargo:rustc-link-lib=static=pdfium");
    }
}

fn link_dynamic() {
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=pdfium.dll");
    } else {
        println!("cargo:rustc-link-lib=dylib=pdfium");
    }
}

/// Pdfium uses a custom C++ standard library that we need to link with.
/// Thankfully it produces all the necessary object files during the pdfium build,
/// we just have to direct the cc crate to use them.
fn link_custom_libcxx() {
    let lib_dir = pdfium_lib_dir();
    let custom_libcxx_dir = lib_dir
        .join("buildtools")
        .join("third_party")
        .join("libc++")
        .join("libc++");
    let object_files = std::fs::read_dir(custom_libcxx_dir)
        .unwrap()
        .map(|r| r.unwrap().path());

    let mut builder = cc::Build::new();
    for object_file in object_files {
        builder.object(object_file);
    }

    builder.compile("custom_libcxx");
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    eprintln!("HELP: If bindgen fails due to missing LLVM, you may want to download and install LLVM. Check the release page on GitHub: https://github.com/llvm/llvm-project/releases");

    use std::path::PathBuf;

    use crate::pdfium_build::path::out_dir;

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
        pdfium_build::path::pdfium_root_dir().join("public")
    };
    builder = builder.clang_arg(format!(
        "-I{}",
        include_path
            .canonicalize()
            .unwrap_or_else(|_| panic!("could not canonicalize include path {:?}", include_path))
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

    // Write the bindings to the OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_dir().join("bindings.rs"))
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
