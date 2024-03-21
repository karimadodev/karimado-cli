mod libgit2;
mod reqwest;

use std::path::{Path, PathBuf};

use crate::{error::*, url::*};

pub(crate) fn download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    match url.scheme {
        Scheme::GitHttps => git_https_download(url, downloads_path),
        Scheme::Https => https_download(url, downloads_path),
        Scheme::Http => http_download(url, downloads_path),
    }
}

fn git_https_download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    libgit2::download(url, downloads_path)
}

fn https_download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    reqwest::download(url, downloads_path)
}

fn http_download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    reqwest::download(url, downloads_path)
}
