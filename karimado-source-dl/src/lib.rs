mod archive;
mod backend;
mod contrib;
mod downloader;
mod error;
mod url;

use url::Url;

pub use downloader::Downloader;
pub use error::{Error, Result};

pub fn download(url: &str) -> Result<std::path::PathBuf> {
    Downloader::new().download(url)
}
