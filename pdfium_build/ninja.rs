use super::{depot_tools, path};

pub fn compile() {
    depot_tools::cmd("ninja")
        .args(["-C", "out/Default"])
        .current_dir(path::pdfium_root_dir())
        .status()
        .expect("error running ninja command");
}
