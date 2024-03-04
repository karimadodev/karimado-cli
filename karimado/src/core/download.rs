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

pub(crate) fn download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    Ok(match Scheme::from_str(url.scheme()) {
        Ok(scheme) => match scheme {
            Scheme::GitHttps => git_https::download(url, downloads_path)?,
            Scheme::Https => https::download(url, downloads_path)?,
        },
        Err(..) => anyhow_bail_unknown_scheme(url.scheme())?,
    })
}

fn anyhow_bail_unknown_scheme(scheme: &str) -> Result<PathBuf> {
    let values = Vec::from_iter(Scheme::iter().map(|v| v.to_string()));
    anyhow::bail!(
        "the scheme was expected one of {:?} but got {:?}",
        values,
        scheme
    )
}
