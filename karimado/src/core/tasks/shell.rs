use std::process::Command;

#[cfg(unix)]
pub(crate) fn command(c: &str) -> Command {
    let mut cmd = Command::new("sh");
    cmd.args(["-c", &c]);
    cmd
}

#[cfg(window)]
pub(crate) fn command(c: &str) -> Command {
    let mut cmd = Command::new("cmd.exe");
    cmd.args(["/c", &c]);
    cmd
}
