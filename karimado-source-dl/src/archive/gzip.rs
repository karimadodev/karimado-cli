#[cfg(test)]
#[path = "gzip_test.rs"]
mod tests;

use flate2::read::GzDecoder;
use std::fs::File;
use std::path::Path;
use tar::Archive;

use crate::error::*;
use SourceDecompressErrorKind::IoError;

pub(crate) fn decompress(archive_path: &Path, target_path: &Path) -> Result<()> {
    let file = File::open(archive_path).map_err(IoError)?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    archive.unpack(target_path).map_err(IoError)?;
    Ok(())
}
