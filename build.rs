// Copyright 2021 pdfium-sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(feature = "bindgen")]
extern crate bindgen;

fn main() {
    // Tell cargo to tell rustc to link the system
    // shared library.
    #[cfg(windows)]
    {
        println!("cargo:rustc-link-lib=dylib=pdfium.dll");
    }

    #[cfg(not(windows))]
    {
        println!("cargo:rustc-link-lib=dylib=pdfium");
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    #[cfg(feature = "bindgen")]
    {
        println!("cargo:rerun-if-changed=wrapper.h");

        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("wrapper.h")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Try to keep original comments for docs
            .clang_args(
                [
                    "-fretain-comments-from-system-headers",
                    "-fparse-all-comments",
                ]
                .iter(),
            )
            .generate_comments(true)
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = std::path::PathBuf::from("src");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
