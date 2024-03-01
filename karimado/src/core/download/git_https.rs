use anyhow::Result;
use fastrand::alphanumeric;
use git2::Repository;
use std::{
    iter,
    path::{Path, PathBuf},
};
use url::Url;

pub(crate) fn download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    let url = String::from(url.clone());
    let url = url.replace("git+https://", "https://");

    let path: String = iter::repeat_with(alphanumeric).take(8).collect();
    let path = downloads_path.join(path);
    Repository::clone(&url, &path)?;
    Ok(path)
}
