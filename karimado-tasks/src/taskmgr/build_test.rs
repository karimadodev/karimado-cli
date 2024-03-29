use super::*;

fn load_tasks_from(taskfile: &str, cli_args: &[String]) -> Result<Vec<Task>> {
    let current_dir = std::env::current_dir().unwrap();
    build(&current_dir, taskfile, cli_args)
}

fn load_tasks_from_ok_taskfile() -> Vec<Task> {
    let taskfile = "tests/fixtures/taskmgr/build/ok-full/tasks.toml";
    let cli_args = vec![String::from("--check"), String::from("-v")];
    load_tasks_from(taskfile, &cli_args).unwrap()
}

#[test]
fn ok_task_name() {
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
fn ok_task_command() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:install").unwrap();
    assert_eq!(task.command, "cargo install");
}

#[test]
fn ok_task_command_vars_cli_args() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:fmt").unwrap();
    assert_eq!(task.command, "cargo fmt -- --check -v");
}

#[test]
fn ok_task_description() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:install").unwrap();
    assert_eq!(task.description, Some("Install a Rust binary".to_string()));
}

#[test]
fn ok_task_description_none() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "ruby:true").unwrap();
    assert_eq!(task.description, None);
}

#[test]
fn ok_task_current_dir() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "ruby:true").unwrap();
    assert_eq!(task.current_dir, std::env::current_dir().unwrap());
}

#[test]
fn ok_task_current_dir_using_include_working_dir() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:install").unwrap();
    let actual = task.current_dir.clone();
    let expected = std::env::current_dir().unwrap().join("dir-a1/dir-b1");
    assert_eq!(actual, expected);
}

#[test]
fn ok_task_current_dir_using_include_working_dir_nested() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:run:new").unwrap();
    let actual = task.current_dir.clone();
    let expected = std::env::current_dir().unwrap().join("dir-a2");
    assert_eq!(actual, expected);
}

#[test]
fn ok_task_current_dir_using_working_dir() {
    let tasks = load_tasks_from_ok_taskfile();
    let task = tasks.iter().find(|t| t.name == "cargo:run:build").unwrap();
    let actual = task.current_dir.clone();
    let expected = std::env::current_dir().unwrap().join("dir-a3/dir-b1");
    assert_eq!(actual, expected);
}

#[test]
fn err_include_nonexists() {
    let taskfile = "tests/fixtures/taskmgr/build/err-include-nonexists/tasks.toml";
    let r = load_tasks_from(taskfile, &[]);
    assert!(r.is_err());
    assert!(matches!(r, Err(Error::TaskFileParseError(_))));

    let e = r.unwrap_err().to_string();
    assert!(e.contains("taskfile `tasks/nonexists.toml` does not exists under"));
}

#[test]
fn ok_hbs_helper_getvar_or_default() {
    let mut vars = JsonMap::new();
    vars.insert("ARG_1".to_string(), json!(""));
    vars.insert("ARG_2".to_string(), json!("abc"));

    let mut renderer = handlebars::Handlebars::new();
    let helper = Box::new(hbs_helper_getvar_or_default(vars.clone()));
    renderer.register_helper("getvar_or_default", helper);

    let r = renderer
        .render_template("{{ getvar_or_default 'ARG_1' '123' }}", &vars)
        .unwrap();
    assert_eq!(r, "123");

    let r = renderer
        .render_template("{{ getvar_or_default 'ARG_2' '456' }}", &vars)
        .unwrap();
    assert_eq!(r, "abc");

    let e = renderer
        .render_template("{{ getvar_or_default 'ARG_3' '456' }}", &vars)
        .unwrap_err()
        .to_string();
    assert!(e.contains("var `ARG_3` is undefined"));
}
