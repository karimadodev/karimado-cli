use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Task {
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) command: String,
}
