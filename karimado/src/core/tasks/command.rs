use std::path::{Path, PathBuf};

#[derive(Default)]
pub(crate) struct Command {
    command: String,
    current_dir: PathBuf,
}

impl Command {
    pub(crate) fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            ..Default::default()
        }
    }

    pub(crate) fn current_dir(&mut self, current_dir: &Path) -> &mut Self {
        self.current_dir = current_dir.to_path_buf();
        self
    }

    pub(crate) fn spawn(&self) -> std::io::Result<std::process::Child> {
        self.command().current_dir(&self.current_dir).spawn()
    }

    #[cfg(unix)]
    fn command(&self) -> std::process::Command {
        let mut cmd = std::process::Command::new("sh");
        cmd.args(["-c", &self.command]);
        cmd
    }

    #[cfg(window)]
    fn command(&self) -> std::process::Command {
        let mut cmd = std::process::Command::new("cmd.exe");
        cmd.args(["/c", &self.command]);
        cmd
    }
}
