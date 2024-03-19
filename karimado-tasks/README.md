# karimado-cli#karimado-tasks

## Installation

Add following line to `Cargo.toml` file:

```toml
[dependencies]
karimado-tasks = "0.0"
```

## Usage

```rust
fn main() -> Result<(), karimado_tasks::Error> {
    let args: Vec<_> = std::env::args().collect();
    let taskmgr = karimado_tasks::TaskMgr::builder()
        .current_dir(&std::env::current_dir().unwrap())
        .taskfile("examples/karimado/tasks.toml")
        .build()
        .unwrap();
    taskmgr.execute(&args[1..], || None)
}
```

More examples can be found in [examples](./examples).

## License

The crate is available as open source under the terms of the [LGPL-3.0-only License](./LICENSE).
