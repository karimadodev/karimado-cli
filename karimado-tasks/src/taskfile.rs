mod include;
mod task;

use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::error::*;

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
    let data = fs::read_to_string(path).map_err(TaskFileParseFailedKind::IoError)?;
    let taskfile = toml::from_str(&data).map_err(TaskFileParseFailedKind::TomlError)?;
    Ok(taskfile)
}
