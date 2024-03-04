mod zip;

use anyhow::Result;
use std::{path::Path, str::FromStr};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(Display, EnumIter, EnumString)]
enum MimeType {
    #[strum(serialize = "application/zip")]
    Zip,
}

pub(crate) fn decompress(archive_ppath: &Path, target_path: &Path) -> Result<()> {
    match infer::get_from_path(archive_ppath)? {
        Some(info) => match MimeType::from_str(info.mime_type()) {
            Ok(mime_type) => match mime_type {
                MimeType::Zip => zip::decompress(archive_ppath, target_path)?,
            },
            Err(..) => anyhow_bail_unknown_mime_type(info.mime_type())?,
        },
        None => anyhow_bail_unknown_mime_type("unknown")?,
    };
    Ok(())
}

fn anyhow_bail_unknown_mime_type(mime_type: &str) -> Result<()> {
    let values = Vec::from_iter(MimeType::iter().map(|v| v.to_string()));
    anyhow::bail!(
        "the MIME type was expected one of {:?} but got {:?}",
        values,
        mime_type
    )
}
