use anyhow::Result;
use std::path::{Path, PathBuf};
use url::Url;

pub(crate) fn download(_: &Url, _: &Path) -> Result<PathBuf> {
    Ok(PathBuf::new())
}
