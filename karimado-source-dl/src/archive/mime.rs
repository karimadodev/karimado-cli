use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::error::*;
use SourceDecompressErrorKind::UnknownMimeType;

#[derive(Display, EnumIter, EnumString)]
pub(crate) enum DecompressType {
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
