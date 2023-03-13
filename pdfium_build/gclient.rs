use super::{depot_tools, path};

pub fn config() {
    if !path::src_dir().join("BUILD.gn").exists() {
        panic!(concat!(
            "Could not find pdfium sources. ",
            "Make sure git submodules are been updated: \n",
            "`git submodule update --init --recursive`."
        ));
    }

    assert!(
        depot_tools::cmd("gclient")
            .args([
                "config",
                "--unmanaged",
                &format!("file://{}", &path::src_dir().display()),
                "--custom-var=checkout_configuration=minimal",
                "--cache-dir"
            ])
            .arg(path::cache_dir())
            .current_dir(path::gclient_build_dir())
            .status()
            .unwrap()
            .success(),
        "error executing gclient config"
    );
}

pub fn sync() {
    assert!(
        depot_tools::cmd("gclient")
            .args(["sync", "--no-history", "--shallow"])
            .current_dir(path::gclient_build_dir())
            .status()
            .unwrap()
            .success(),
        "error executing gclient sync"
    );
}
