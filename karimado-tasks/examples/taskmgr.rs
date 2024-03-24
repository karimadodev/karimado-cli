use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Lists tasks with description of current taskfile
    #[arg(long, short)]
    list: bool,

    /// Executes tasks provided on command line in parallel
    #[arg(long, short)]
    parallel: bool,

    /// Task Name
    #[arg(default_value = "default")]
    task: Vec<String>,

    /// Task Args
    #[arg(raw = true)]
    args: Vec<String>,
}

fn main() -> Result<(), karimado_tasks::Error> {
    // Ctrl-C handler
    let terminated = Arc::new(AtomicBool::new(false));
    let watched_terminated = Arc::clone(&terminated);
    let watched_terminated_value = move || {
        if watched_terminated.load(Ordering::SeqCst) {
            Some("received Ctrl-C signal".to_string())
        } else {
            None
        }
    };
    ctrlc::set_handler(move || terminated.store(true, Ordering::SeqCst))
        .expect("failed to set Ctrl-C handler");

    // cli
    let cli = Cli::parse();
    let taskmgr = karimado_tasks::TaskMgr::builder()
        .current_dir(&std::env::current_dir().unwrap())
        .taskfile("examples/karimado/tasks.toml")
        .cli_args(&cli.args)
        .build()?;

    // cli arg: --list
    if cli.list || cli.task.is_empty() {
        taskmgr.list();
        return Ok(());
    }

    // cli arg: --parallel
    if cli.parallel {
        taskmgr.parallel_execute(&cli.task, watched_terminated_value)
    } else {
        taskmgr.execute(&cli.task, watched_terminated_value)
    }
}
