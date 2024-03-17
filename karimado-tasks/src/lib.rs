#[cfg(test)]
mod tests;

mod error;
mod shell;
mod task;
mod taskfile;
mod taskmgr;

use task::Task;

pub use error::{Error, Result};
pub use taskmgr::TaskMgr;
