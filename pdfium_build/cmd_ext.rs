use std::process::Command;

/// An extension trait to run a command and panic if the status code was not 0.
pub(crate) trait CmdExt {
    #[track_caller]
    fn run_or_panic(&mut self);
}

impl CmdExt for Command {
    #[track_caller]
    fn run_or_panic(&mut self) {
        let dir = self
            .get_current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default();

        println!("Running {:?} in {}", self, dir);
        let status = self
            .status()
            .unwrap_or_else(|e| panic!("failed to run command {:?}: {}", self, e));

        if !status.success() {
            panic!(
                "command {:?} running in {} returned non-zero exit code: {}",
                self, dir, status
            );
        }
    }
}
