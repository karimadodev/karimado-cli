use super::*;

#[test]
fn ok_taskfile_full() {
    let current_dir = std::env::current_dir().unwrap();
    let taskfile = "tests/fixtures/ok-taskfile-full/tasks.toml";
    let cli_args = vec![String::from("--check"), String::from("-v")];

    let r = build(&current_dir, taskfile, &cli_args);
    assert!(r.is_ok());

    // tasks's order
    let tasks = r.unwrap();
    assert_eq!(
        tasks
            .iter()
            .map(|task| task.name.clone())
            .collect::<Vec<_>>(),
        vec![
            "cargo:build",
            "cargo:fmt",
            "cargo:install",
            "cargo:new",
            "cargo:run",
            "cargo:run:build",
            "cargo:run:new",
            "cargo:run:run",
            "cargo:run:scaffold:install",
            "ruby:false",
            "ruby:true",
        ],
    );

    // task's name, command, description
    let task = tasks.iter().find(|t| t.name == "cargo:install").unwrap();
    assert_eq!(task.command, "cargo install");
    assert_eq!(task.description, Some("Install a Rust binary".to_string()));
    assert_eq!(task.current_dir, current_dir);

    // task's name, command, description
    let task = tasks.iter().find(|t| t.name == "ruby:true").unwrap();
    assert_eq!(task.command, "ruby -e 'exit(0)'");
    assert_eq!(task.description, None);
    assert_eq!(task.current_dir, current_dir);

    // taskfile::Include#working_dir
    let task = tasks.iter().find(|t| t.name == "cargo:run:new").unwrap();
    assert_eq!(task.current_dir, current_dir.join("path-relative-to-cwd"));

    // taskfile::Task#command builtin vars
    let task = tasks.iter().find(|t| t.name == "cargo:fmt").unwrap();
    assert_eq!(task.command, "cargo fmt -- --check -v");
}

#[test]
fn err_taskfile_include_nonexists() {
    let current_dir = std::env::current_dir().unwrap();
    let taskfile = "tests/fixtures/err-taskfile-include-nonexists/tasks.toml";
    let cli_args = vec![];

    let r = build(&current_dir, taskfile, &cli_args);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::TaskFileParseFailed(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.contains("taskfile `tasks/nonexists.toml` does not exists under"));
}
