use anyhow::Result;
use std::path::{Path, PathBuf};

static CONFIG_FILE_NAME: &str = "karimado.toml";

pub(crate) fn root_path() -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;
    loop {
        if path.join(CONFIG_FILE_NAME).exists() {
            return Ok(path);
        }
        if !path.pop() {
            anyhow::bail!("could not locate {:?}", CONFIG_FILE_NAME)
        }
    }
}

pub(crate) fn config_file_path(root_path: &Path) -> PathBuf {
    root_path.join(CONFIG_FILE_NAME)
}
