use anyhow::Result;
use clap::builder::styling::{AnsiColor, Styles};
use std::path::PathBuf;

pub(crate) fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

pub(crate) fn root_path() -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;
    loop {
        if path.join("karimado.toml").exists() {
            return Ok(path);
        }
        if !path.pop() {
            anyhow::bail!("could not locate {:?}", "karimado.toml")
        }
    }
}
