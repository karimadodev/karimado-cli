#[cfg(test)]
#[path = "downloader_test.rs"]
mod tests;

use std::env;
use std::path::{Path, PathBuf};

use crate::{backend, error::*, Url};

#[derive(Default)]
pub struct Downloader {
    current_dir: Option<PathBuf>,
    downloads_dir: Option<PathBuf>,
}

impl Downloader {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn current_dir(&mut self, current_dir: &Path) -> &mut Self {
        self.current_dir = Some(current_dir.to_path_buf());
        self
    }

    pub fn downloads_dir(&mut self, downloads_dir: &Path) -> &mut Self {
        self.downloads_dir = Some(downloads_dir.to_path_buf());
        self
    }

    pub fn download(&self, url: &str) -> Result<PathBuf> {
        let url = Url::parse_with_quirks_mode(url, self.current_dir.clone())?;
        let downloads_dir = self.downloads_dir.clone().unwrap_or(env::temp_dir());
        backend::download(&url, &downloads_dir)
    }
}
