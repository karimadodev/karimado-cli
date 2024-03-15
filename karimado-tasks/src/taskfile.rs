mod include;
mod task;

use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::Path};

pub(crate) use include::*;
pub(crate) use task::*;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Taskfile {
    #[serde(default)]
    pub(crate) includes: Vec<Include>,
    #[serde(default)]
    pub(crate) tasks: Vec<Task>,
}

pub(crate) fn from_taskfile(path: &Path) -> Result<Taskfile> {
    let data = fs::read_to_string(path)?;
    Ok(toml::from_str(&data)?)
}
