mod gzip;
mod zip;

use std::path::Path;
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::error::*;
use SourceDecompressErrorKind::{IoError, UnknownMimeType};

#[derive(Display, EnumIter, EnumString)]
pub(crate) enum DecompressType {
    #[strum(serialize = "application/gzip")]
    Gzip,
    #[strum(serialize = "application/zip")]
    Zip,
}

impl DecompressType {
    pub(crate) fn parse(str: &str) -> Result<Self> {
        DecompressType::from_str(str).map_err(|_| {
            let v = Vec::from_iter(DecompressType::iter().map(|v| v.to_string()));
            let e = format!(
                "the decomporess type was expected one of {:?} but got {:?}",
                v, str
            );
            Error::SourceDecompressError(UnknownMimeType(e))
        })
    }
}

pub(crate) fn decompress(archive_path: &Path, target_path: &Path) -> Result<()> {
    let kind = infer::get_from_path(archive_path).map_err(IoError)?;
    let kind = kind.map(|k| k.mime_type()).unwrap_or("unknown");
    match DecompressType::parse(kind)? {
        DecompressType::Gzip => gzip::decompress(archive_path, target_path),
        DecompressType::Zip => zip::decompress(archive_path, target_path),
    }
}
