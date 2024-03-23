#[test]
fn examples() {
    trycmd::TestCases::new()
        .register_bins(trycmd::cargo::compile_examples([]).unwrap())
        .case("examples/taskmgr-*.md")
        .case("examples/taskmgr.md");
}

#[cfg(unix)]
#[test]
fn examples_taskmgr_ctrlc() {
    use escargot::CargoBuild;
    use nix::sys::signal;
    use nix::sys::signal::Signal;
    use nix::unistd::Pid;
    use std::process::Stdio;
    use std::thread;
    use std::time::Duration;

    let mut cmd = CargoBuild::new()
        .example("taskmgr")
        .run()
        .unwrap()
        .command();
    let child = cmd
        .args(["-p", "sleepn", "sleepn", "--", "4"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    thread::sleep(Duration::from_millis(1000));
    signal::kill(Pid::from_raw(child.id() as i32), Signal::SIGINT).unwrap();

    let output = child.wait_with_output().unwrap();
    let status = output.status;
    assert!(!status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains(" sleepn.1 | -> ruby -e 'sleep(4)'"));
    assert!(stderr.contains(" sleepn.2 | -> ruby -e 'sleep(4)'"));
    assert!(stderr.contains(" sleepn.1 | <> task terminated"));
    assert!(stderr.contains(" sleepn.2 | <> task terminated"));
    assert!(stderr.contains("TaskRunError(\"received Ctrl-C signal\")"))
}
