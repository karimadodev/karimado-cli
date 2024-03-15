use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct Task {
    pub(crate) name: String,
    pub(crate) command: String,
    pub(crate) description: Option<String>,
    pub(crate) current_dir: PathBuf,
}
