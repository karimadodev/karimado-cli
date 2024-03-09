mod include;
mod task;

use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Taskfile {
    #[serde(default)]
    pub(crate) includes: Vec<include::Include>,
    #[serde(default)]
    pub(crate) tasks: Vec<task::Task>,
}

pub(crate) fn from_taskfile(path: &Path) -> Result<Taskfile> {
    let data = fs::read_to_string(path)?;
    Ok(toml::from_str(&data)?)
}
