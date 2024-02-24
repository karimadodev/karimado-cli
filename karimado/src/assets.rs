use anyhow::Result;
use rust_embed::RustEmbed;
use std::{fs, path::Path};

#[derive(RustEmbed)]
#[folder = "src/assets"]
struct Assets;

pub(crate) fn copy(from: &str, to: &Path) -> Result<()> {
    let f = Assets::get(from).unwrap();
    fs::write(to, f.data)?;
    Ok(())
}
