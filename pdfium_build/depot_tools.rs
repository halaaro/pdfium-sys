use super::{cmd_ext::CmdExt, path::depot_tools_dir};
use std::{env, process::Command};

pub(crate) fn clone() {
    if depot_tools_dir().exists() {
        return;
    }

    Command::new("git")
        .args([
            "clone",
            "https://chromium.googlesource.com/chromium/tools/depot_tools.git",
        ])
        .arg(depot_tools_dir())
        .run_or_panic()
}

pub(crate) fn cmd(name: &str) -> std::process::Command {
    let tools_path = depot_tools_dir();
    let tools_dir = tools_path
        .to_str()
        .expect("depot_tools directory was not UTF-8");

    let path_separator = if cfg!(windows) { ';' } else { ':' };

    let new_path = format!(
        "{}{}{}",
        tools_dir,
        path_separator,
        env::var("PATH").unwrap_or_default()
    );

    let name = if cfg!(windows) {
        format!("{}.bat", name)
    } else {
        name.to_string()
    };

    let mut cmd = Command::new(name);
    cmd.env("PATH", new_path)
        .env("DEPOT_TOOLS_WIN_TOOLCHAIN", "0");
    cmd
}
