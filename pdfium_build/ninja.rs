use super::{cmd_ext::CmdExt, depot_tools, path};

pub(crate) fn compile() {
    depot_tools::cmd("ninja")
        .args(["-C", "out/Default"])
        .current_dir(path::pdfium_root_dir())
        .run_or_panic()
}
