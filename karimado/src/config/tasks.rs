use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Tasks {
    pub(crate) taskfile: String,
}
