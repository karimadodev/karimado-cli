use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct Task {
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) command: String,
    pub(crate) current_dir: PathBuf,
}
