use anyhow::Result;
use rust_embed::RustEmbed;
use std::{fs, path::Path};

#[derive(RustEmbed)]
#[folder = "src/assets"]
struct Assets;

pub(crate) fn copy(from: &str, to: &Path) -> Result<()> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent)?;
    }
    let f = Assets::get(from).expect("asset file does not exists");
    fs::write(to, f.data)?;
    Ok(())
}
