use super::{depot_tools, path};

pub fn compile() {
    assert!(
        depot_tools::cmd("ninja")
            .arg("-C")
            .arg(path::pdfium_out_dir())
            .current_dir(path::pdfium_root_dir())
            .status()
            .unwrap()
            .success(),
        "error running ninja command"
    );
}
