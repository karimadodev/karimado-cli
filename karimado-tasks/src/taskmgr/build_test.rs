use super::*;

fn load_tasks_from(taskfile: &str, cli_args: &[String]) -> Result<Vec<Task>> {
    let current_dir = std::env::current_dir().unwrap();
    build(&current_dir, taskfile, cli_args)
}

fn load_tasks_from_ok_taskfile() -> Vec<Task> {
    let taskfile = "tests/fixtures/taskmgr/build/ok-taskfile-full/tasks.toml";
    let cli_args = vec![String::from("--check"), String::from("-v")];
    load_tasks_from(taskfile, &cli_args).unwrap()
}

#[test]
fn ok_taskfile_name() {
    let tasks = load_tasks_from_ok_taskfile();
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
}

#[test]
fn ok_taskfile_command() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:install").unwrap();
    assert_eq!(task.command, "cargo install");
}

#[test]
fn ok_taskfile_command_vars_cli_args() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:fmt").unwrap();
    assert_eq!(task.command, "cargo fmt -- --check -v");
}

#[test]
fn ok_tasks_description() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:install").unwrap();
    assert_eq!(task.description, Some("Install a Rust binary".to_string()));
}

#[test]
fn ok_tasks_description_none() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "ruby:true").unwrap();
    assert_eq!(task.description, None);
}

#[test]
fn ok_tasks_current_dir() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "ruby:true").unwrap();
    assert_eq!(task.current_dir, std::env::current_dir().unwrap());
}

#[test]
fn ok_tasks_current_dir_includes_working_dir() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:install").unwrap();
    let actual = task.current_dir.clone();
    let expected = std::env::current_dir().unwrap().join("dir-a1/dir-b1");
    assert_eq!(actual, expected);
}

#[test]
fn ok_tasks_current_dir_includes_working_dir_nested() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:run:new").unwrap();
    let actual = task.current_dir.clone();
    let expected = std::env::current_dir().unwrap().join("dir-a2");
    assert_eq!(actual, expected);
}

#[test]
fn ok_tasks_current_dir_command_working_dir() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:run:build").unwrap();
    let actual = task.current_dir.clone();
    let expected = std::env::current_dir().unwrap().join("dir-a3/dir-b1");
    assert_eq!(actual, expected);
}

#[test]
fn err_taskfile_include_nonexists() {
    let taskfile = "tests/fixtures/taskmgr/build/err-taskfile-include-nonexists/tasks.toml";
    let r = load_tasks_from(taskfile, &[]);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::TaskFileParseFailed(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.contains("taskfile `tasks/nonexists.toml` does not exists under"));
}
