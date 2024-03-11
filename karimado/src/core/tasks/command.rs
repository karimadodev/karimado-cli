use std::process;

pub(crate) struct Command {
    command: String,
}

impl Command {
    pub(crate) fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
        }
    }

    #[cfg(unix)]
    pub(crate) fn spawn(&self) -> std::io::Result<process::Child> {
        process::Command::new("/bin/sh")
            .arg("-c")
            .arg(&self.command)
            .spawn()
    }

    #[cfg(window)]
    pub(crate) fn spawn(&self) -> std::io::Result<process::Child> {
        process::Command::new("cmd.exe")
            .arg("/c")
            .arg(&self.command)
            .spawn()
    }
}
