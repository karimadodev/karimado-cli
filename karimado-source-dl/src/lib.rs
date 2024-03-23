#[cfg(test)]
mod tests;

mod archive;
mod backend;
mod contrib;
mod downloader;
mod error;
mod url;

use url::Url;

pub use downloader::Downloader;
pub use error::{Error, Result};
