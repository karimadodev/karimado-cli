fn main() {
    let taskmgr = karimado_tasks::TaskMgr::builder()
        .current_dir(&std::env::current_dir().unwrap())
        .taskfile("examples/karimado/tasks.toml")
        .build()
        .unwrap();
    taskmgr.list()
}
