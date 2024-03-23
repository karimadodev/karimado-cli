#[cfg(test)]
#[path = "downloader_test.rs"]
mod tests;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{backend, error::*, Download, Url};
use SourceDownloadErrorKind::IoError;

#[derive(Default)]
pub struct Downloader {
    downloads_path: Option<PathBuf>,
}

impl Downloader {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn downloads_path(&mut self, downloads_path: &Path) -> &mut Self {
        self.downloads_path = Some(downloads_path.to_path_buf());
        self
    }

    pub fn download(&self, download: &Download) -> Result<PathBuf> {
        let url = Url::parse_with_quirks_mode(&download.url, None)?;
        let downloads_path = self.downloads_path.clone().unwrap_or(env::temp_dir());
        let path = backend::download(&url, &downloads_path)?;
        if let Some(dirname) = &download.dirname {
            let target_path = downloads_path.join(dirname);
            fs::rename(path, target_path.clone()).map_err(IoError)?;
            Ok(target_path)
        } else {
            Ok(path)
        }
    }
}
