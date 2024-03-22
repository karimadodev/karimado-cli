mod mime;
mod zip;

use std::path::Path;

use crate::error::*;
use SourceDecompressErrorKind::IoError;

pub(crate) fn decompress(archive_path: &Path, target_path: &Path) -> Result<()> {
    let kind = infer::get_from_path(archive_path).map_err(IoError)?;
    let kind = kind.map(|k| k.mime_type()).unwrap_or("unknown");
    match mime::DecompressType::parse(kind)? {
        mime::DecompressType::Zip => zip::decompress(archive_path, target_path),
    }
}
