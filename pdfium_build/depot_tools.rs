use super::path;
use std::{env, process::Command};

pub(crate) fn cmd(name: &str) -> std::process::Command {
    let name = if cfg!(windows) {
        format!("{}.bat", name)
    } else {
        name.to_string()
    };

    let mut tools = path::repo_dir();
    tools.push("depot_tools");
    let tools_dir = tools.to_str().expect("tools directory was not UTF-8");

    let path_separator = if cfg!(windows) { ';' } else { ':' };

    let new_path = format!(
        "{}{}{}",
        tools_dir,
        path_separator,
        env::var("PATH").unwrap_or_default()
    );

    let mut cmd = Command::new(name);
    cmd.env("PATH", new_path)
        .env("DEPOT_TOOLS_WIN_TOOLCHAIN", "0");
    cmd
}
