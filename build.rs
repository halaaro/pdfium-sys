use std::{
    env,
    path::{Path, PathBuf},
};

use flate2::bufread::GzDecoder;
use tar::Archive;

extern crate bindgen;

const PDFIUM_DOWNLOAD_URL: &str =
    "https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F5567/pdfium-win-x64.tgz";

fn main() {
    let out_dir = get_pdfium();

    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search={}",
        out_dir.join("pdfium/lib").display()
    );

    // Tell cargo to tell rustc to link pdfium.
    #[cfg(not(windows))]
    println!("cargo:rustc-link-lib=pdfium");

    #[cfg(windows)]
    println!("cargo:rustc-link-lib=pdfium.dll");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    if !out_dir.join("bindings.rs").exists() {
        println!("Generating bindings...");
        generate_bindings(&out_dir);
    }
}

fn get_pdfium() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    if !out_dir.join("pdfium").exists() {
        println!(
            "Pdfium not found - downloading from https://github.com/bblanchon/pdfium-binaries"
        );
        download_pdfium(&out_dir);
    }
    out_dir
}

fn download_pdfium(out_dir: &Path) {
    let client = reqwest::blocking::Client::new();
    let tar_gz = client
        .get(PDFIUM_DOWNLOAD_URL)
        .send()
        .expect("failed to send PDFIUM request")
        .bytes()
        .expect("failed to receive PDFIUM");

    let tar = GzDecoder::new(&*tar_gz);
    let mut archive = Archive::new(tar);
    archive
        .unpack(out_dir.join("pdfium"))
        .expect("failed to unpack tar archive");
}

fn generate_bindings(out_dir: &Path) {
    eprintln!("HELP: If bindgen fails, you may want to download and install LLVM. Check the release page on GitHub: https://github.com/llvm/llvm-project/releases");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Need to tell Clang to search the pdfium files.
    let include_path = out_dir.join("pdfium/include");
    println!("Adding {} to search path.", include_path.display());
    std::env::set_var("CPATH", include_path);

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
        // Finish the builder and generate the bindings.
        .generate()
        .expect("failed to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
