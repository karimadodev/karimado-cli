mod error;
mod shell;
mod task;
mod taskfile;
mod taskmgr;

use task::Task;

pub use error::{Error, Result};
pub use taskmgr::TaskMgr;

#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn global_setup() {
        env_logger::builder().is_test(true).init();
    }
}
