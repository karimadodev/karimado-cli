mod tasks;
mod workspace;

use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    pub(crate) workspace: workspace::Workspace,
    pub(crate) tasks: tasks::Tasks,
}

pub(crate) fn from_config_file(path: &Path) -> Result<Config> {
    let data = fs::read_to_string(path)?;
    Ok(toml::from_str(&data)?)
}
