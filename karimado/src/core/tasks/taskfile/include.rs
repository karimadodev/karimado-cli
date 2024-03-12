use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Include {
    pub(crate) name: String,
    pub(crate) taskfile: String,
    #[serde(default)]
    pub(crate) optional: bool,
}
