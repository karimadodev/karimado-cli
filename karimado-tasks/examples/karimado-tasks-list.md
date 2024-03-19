# taskmgr#list

Lists tasks with description of current taskfile:

```rust
fn main() {
    env_logger::builder()
        .format_level(false)
        .format_target(false)
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let taskmgr = karimado_tasks::TaskMgr::builder()
        .current_dir(&std::env::current_dir().unwrap())
        .taskfile("examples/karimado/tasks.toml")
        .build()
        .unwrap();
    taskmgr.list()
}
```

## List tasks

```console
$ karimado-tasks-list
Available tasks for this project:
* cargo:build   # Compile the current package
* cargo:fmt     # Formats all files
* cargo:install # Install a Rust binary
* cargo:new     # Create a new cargo package
* cargo:run     # Run a binary or example of the local package
* false         # ruby -e 'exit(1)'
* sleep1        # ruby -e 'sleep(1)'
* true          # ruby -e 'exit(0)'

```
