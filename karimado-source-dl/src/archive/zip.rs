#[cfg(test)]
#[path = "zip_test.rs"]
mod tests;

use std::fs::File;
use std::path::Path;

use crate::error::*;
use SourceDecompressErrorKind::{IoError, ZipError};

pub(crate) fn decompress(archive_path: &Path, target_path: &Path) -> Result<()> {
    let file = File::open(archive_path).map_err(IoError)?;
    let mut archive = zip::ZipArchive::new(file).map_err(ZipError)?;
    archive.extract(target_path).map_err(ZipError)?;
    Ok(())
}
