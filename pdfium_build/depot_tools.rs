use super::path;
use std::{env, process::Command};

pub fn cmd(name: &str) -> std::process::Command {
    let mut tools = path::repo_dir();
    tools.push("depot_tools");
    let sep = if cfg!(windows) { ';' } else { ':' };
    let new_path = format!(
        "{}{sep}{}",
        tools.display(),
        env::var("PATH").unwrap_or_default()
    );
    let mut cmd = Command::new(name);
    cmd.env("PATH", new_path).env("DEPOT_TOOLS_UPDATE", "0");
    cmd
}
