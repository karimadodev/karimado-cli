mod git_https;
mod https;

use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};
use url::Url;

#[derive(Display, EnumIter, EnumString)]
enum Scheme {
    #[strum(serialize = "git+https")]
    GitHttps,
    #[strum(serialize = "https")]
    Https,
}

pub(crate) fn download(url: &Url, path: &Path) -> Result<PathBuf> {
    Ok(match Scheme::from_str(url.scheme()) {
        Ok(scheme) => match scheme {
            Scheme::GitHttps => git_https::download(url, path)?,
            Scheme::Https => https::download(url, path)?,
        },
        Err(..) => {
            let values = Vec::from_iter(Scheme::iter().map(|v| v.to_string()));
            anyhow::bail!(
                "invalid value: url {:?}, expected one of the given scheme values: {:?}",
                url.as_str(),
                values
            )
        }
    })
}
