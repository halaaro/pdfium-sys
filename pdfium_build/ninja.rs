use super::{depot_tools, path};

pub fn compile() {
    assert!(
        depot_tools::cmd("ninja")
            .args(["-C", &path::pdfium_out_dir().display().to_string()])
            .current_dir(path::pdfium_root_dir())
            .status()
            .unwrap()
            .success(),
        "error running ninja command"
    );
}
