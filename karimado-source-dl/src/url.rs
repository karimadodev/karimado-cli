mod scheme;

use crate::error::*;

pub(crate) use scheme::*;

pub(crate) struct Url {
    pub(crate) scheme: Scheme,
}

impl Url {
    pub(crate) fn parse(_input: &str) -> Result<Self> {
        Ok(Self {
            scheme: Scheme::GitHttps,
        })
    }
}
