#[cfg(test)]
mod tests;

mod backend;
mod contrib;
mod download;
mod downloader;
mod error;
mod url;

use url::Url;

pub use download::Download;
pub use downloader::Downloader;
pub use error::{Error, Result};
