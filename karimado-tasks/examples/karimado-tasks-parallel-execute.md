# taskmgr#parallel_execute

Executes tasks provided on command line in parallel:

```rust
fn main() -> Result<(), karimado_tasks::Error> {
    env_logger::builder()
        .format_level(false)
        .format_target(false)
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .init();

    let args: Vec<_> = std::env::args().collect();
    let taskmgr = karimado_tasks::TaskMgr::builder()
        .current_dir(&std::env::current_dir().unwrap())
        .taskfile("examples/karimado/tasks.toml")
        .build()
        .unwrap();
    taskmgr.parallel_execute(&args[1..], || None)
}
```

## Invoke one task

```console
$ karimado-tasks-parallel-execute ruby:true
 ruby:true | -> ruby -e 'exit(0)'
 ruby:true | task finished

```


## Invoke multiple tasks

```console
$ karimado-tasks-parallel-execute ruby:true ruby:true ruby:true
 ruby:true | -> ruby -e 'exit(0)'
 ruby:true | -> ruby -e 'exit(0)'
 ruby:true | -> ruby -e 'exit(0)'
 ruby:true | task finished
 ruby:true | task finished
 ruby:true | task finished

```


## Immediately terminated if task failed

```console,ignore
$ karimado-tasks-parallel-execute ruby:true ruby:false ruby:true
? 1
  ruby:true | -> ruby -e 'exit(0)'
 ruby:false | -> ruby -e 'exit(1)'
  ruby:true | -> ruby -e 'exit(0)'
  ruby:true | task finished
  ruby:true | task finished
 ruby:false | task exited with code 1
Error: TaskRunFailed("failed to run task `ruby:false`, exited with code 1")

```
