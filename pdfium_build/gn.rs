use super::{depot_tools, path};
use std::{fs, env};
use std::io::Write;

pub fn gen() {
    path::mkdirs(&path::pdfium_out_dir());

    let mut args_path = path::pdfium_out_dir();
    args_path.push("args.gn");

    {
        let mut args_gn = fs::File::create(&args_path).expect("unable to create args.gn");
        let debug = env::var("DEBUG").unwrap();
        let args = [
            "use_goma = false", // Googlers only. Make sure goma is installed and running first.
            &format!("is_debug = {debug}"), // Enable debugging features.
            "pdf_use_skia = false", // to enable experimental Skia backend.
            "pdf_enable_xfa = false", // Set false to remove XFA support (implies JS support).
            "pdf_enable_v8 = false", // Set false to remove Javascript support.
            "pdf_is_standalone = true", // Set for a non-embedded build.
            "is_component_build = false", // Disable component build (Though it should work)
            "pdf_is_complete_lib = true", // added per https://groups.google.com/g/pdfium/c/FUUMa9e1dpk
            "use_custom_libcxx = false", // added per https://github.com/ajrcarey/pdfium-render/issues/53
        ];
        println!("using args {args:?}");
        for line in args {
            writeln!(args_gn, "{}", line).expect("error writing to args.gn");
        }
    }

    assert!(
        depot_tools::cmd("gn")
            .args(["gen", &path::pdfium_out_dir().display().to_string()])
            .current_dir(&path::pdfium_root_dir())
            .status()
            .unwrap()
            .success(),
        "error running gn gen command"
    );
}
