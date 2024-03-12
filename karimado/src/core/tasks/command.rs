#[cfg(unix)]
pub(crate) fn command(c: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("sh");
    cmd.args(["-c", &c]);
    cmd
}

#[cfg(window)]
pub(crate) fn command(c: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("cmd.exe");
    cmd.args(["/c", &c]);
    cmd
}
