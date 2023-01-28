use super::{depot_tools, path};

pub fn config() {
    depot_tools::cmd("gclient")
        .args([
            "config",
            "--unmanaged",
            &format!("file://{}", &path::src_dir().to_string_lossy()),
            "--custom-var=checkout_configuration=minimal",
            "--cache-dir",
            &path::cache_dir().to_string_lossy(),
        ])
        .current_dir(path::gclient_build_dir())
        .status()
        .expect("error executing gclient config");
}

pub fn sync() {
    depot_tools::cmd("gclient")
        .args(["sync", "--no-history", "--shallow"])
        .current_dir(path::gclient_build_dir())
        .status()
        .expect("error executing gclient sync");
}
