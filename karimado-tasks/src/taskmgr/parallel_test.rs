use std::path::PathBuf;

use super::*;

#[test]
fn ok() {
    let tasks: Vec<Task> = vec![Task {
        command: "cargo version".to_string(),
        current_dir: PathBuf::from("."),
        ..Default::default()
    }];

    let r = execute(&tasks, false);
    assert!(r.is_ok());
}

#[test]
fn err_command_not_found() {
    let tasks: Vec<Task> = vec![Task {
        command: "command404".to_string(),
        current_dir: PathBuf::from("."),
        ..Default::default()
    }];

    let r = execute(&tasks, false);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::TaskRunFailed(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.clone().contains("failed to run task"));
    assert!(e.clone().contains("exited with code"));
}
