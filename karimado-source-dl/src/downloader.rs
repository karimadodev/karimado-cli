use std::env;
use std::path::{Path, PathBuf};

use crate::{backend, error::*, Download, Url};

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
        let url = Url::parse(&download.url)?;
        let downloads_path = self.downloads_path.clone().unwrap_or(env::temp_dir());
        backend::download(&url, &downloads_path)
    }
}