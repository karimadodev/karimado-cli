#[test]
fn examples() {
    trycmd::TestCases::new()
        .register_bins(trycmd::cargo::compile_examples([]).unwrap())
        .case("examples/taskmgr-*.md")
        .case("examples/taskmgr.md");
}

#[cfg(target_family = "unix")]
#[test]
fn examples_taskmgr_ctrlc() {
    use assert_cmd::prelude::*;
    use nix::sys::signal;
    use nix::sys::signal::Signal;
    use nix::unistd::Pid;
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::{Duration, Instant};

    let mut cmd = Command::cargo_bin("examples/taskmgr").unwrap();
    let now = Instant::now();
    let child = cmd
        .args(["-p", "sleepn", "sleepn", "--", "4"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    thread::sleep(Duration::from_millis(100));
    signal::kill(Pid::from_raw(child.id() as i32), Signal::SIGINT).unwrap();

    let output = child.wait_with_output().unwrap();
    let elapsed = now.elapsed();
    assert!(elapsed < Duration::from_secs(1));

    let status = output.status;
    assert!(!status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains(" sleepn | -> ruby -e 'sleep(4)'"));
    assert!(stderr.contains(" sleepn | task terminated"));
    assert!(stderr.contains("TaskRunFailed(\"received Ctrl-C signal\")"))
}
