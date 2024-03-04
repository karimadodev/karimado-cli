use anyhow::Result;
use std::{fs::File, path::Path};

pub(crate) fn decompress(archive_path: &Path, target_path: &Path) -> Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(target_path)?;
    Ok(())
}
