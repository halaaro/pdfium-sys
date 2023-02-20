//! Windows-specific handling for building pdfium.

use registry::{Data, Hive, Security};

use crate::pdfium_build::{cmd_ext::CmdExt, depot_tools, gclient};

pub(crate) fn init() {
    eprintln!("HELP: If pdfium's build complains about missing Debugging Tools for Windows, go to Apps & Features -> Windows Software Development Kit -> Modify -> Choose \"Change\" -> Tick \"Debugging Tools for Windows\"");

    // Need to initialize gclient to install required dependencies, see https://chromium.googlesource.com/chromium/src/+/main/docs/windows_build_instructions.md
    gclient::init();

    verify_long_paths();
}

/// If the long paths option is not enabled in Windows, the build will likely fail.
/// Let's make sure the user sets the option.
fn verify_long_paths() {
    let fs_keys = Hive::LocalMachine
        .open(
            r"SYSTEM\CurrentControlSet\Control\FileSystem",
            Security::Read,
        )
        .unwrap();
    let long_paths = fs_keys.value("LongPathsEnabled").expect("Long paths registry key does not exists. Enable long paths in Windows, otherwise the build will likely fail. See https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation");

    if !matches!(long_paths, Data::U32(1)) {
        panic!("Long paths is not enabled. Enable long paths in Windows, otherwise the build will likely fail. See https://learn.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation");
    }

    // Need to set the longpaths option for the vendored git client in depot_tools.
    // Otherwise you might get "Filename too long" errors.
    depot_tools::cmd("git")
        .args(["config", "--system", "core.longpaths", "true"])
        .run_or_panic();
}
