use anyhow::Result;
use std::path::{Path, PathBuf};
use url::Url;

pub(crate) fn download(_url: &Url, _downloads_path: &Path) -> Result<PathBuf> {
    Ok(PathBuf::new())
}
