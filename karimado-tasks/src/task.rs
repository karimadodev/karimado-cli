use std::path::PathBuf;

#[derive(Clone, Default)]
pub(crate) struct Task {
    pub(crate) name: String,
    pub(crate) command: String,
    pub(crate) description: Option<String>,
    pub(crate) current_dir: PathBuf,
}
