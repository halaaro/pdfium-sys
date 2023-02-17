use crate::pdfium_build::cmd_ext::CmdExt;

use super::{depot_tools, path};

pub(crate) fn config() {
    depot_tools::cmd("gclient")
        .args([
            "config",
            "--unmanaged",
            "https://pdfium.googlesource.com/pdfium.git",
            "--custom-var=checkout_configuration=minimal",
            "--cache-dir",
            path::cache_dir().to_str().expect("cache dir was not UTF-8"),
        ])
        .current_dir(path::gclient_build_dir())
        .run_or_panic();
}

pub(crate) fn sync() {
    depot_tools::cmd("gclient")
        .args(["sync", "--no-history", "--shallow"])
        .current_dir(path::gclient_build_dir())
        .run_or_panic()
}
