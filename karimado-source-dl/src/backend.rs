mod libgit2;
mod local;
mod reqwest;

use std::path::{Path, PathBuf};

use crate::{error::*, url::*};

pub(crate) fn download(url: &Url, downloads_dir: &Path) -> Result<PathBuf> {
    match url.scheme() {
        Scheme::File => file_download(url, downloads_dir),
        Scheme::GitHttps => git_https_download(url, downloads_dir),
        Scheme::Https => https_download(url, downloads_dir),
        Scheme::Http => http_download(url, downloads_dir),
    }
}

fn file_download(url: &Url, downloads_dir: &Path) -> Result<PathBuf> {
    local::download(url, downloads_dir)
}

fn git_https_download(url: &Url, downloads_dir: &Path) -> Result<PathBuf> {
    libgit2::download(url, downloads_dir)
}

fn https_download(url: &Url, downloads_dir: &Path) -> Result<PathBuf> {
    reqwest::download(url, downloads_dir)
}

fn http_download(url: &Url, downloads_dir: &Path) -> Result<PathBuf> {
    reqwest::download(url, downloads_dir)
}
