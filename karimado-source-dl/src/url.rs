#[cfg(test)]
#[path = "url_test.rs"]
mod tests;

use std::env;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::error::*;
use UrlParseErrorKind::{InvalidFilePath, IoError, UnknownScheme, UrlParseError};

#[derive(Clone, Debug, Display, EnumIter, EnumString)]
pub(crate) enum Scheme {
    #[strum(serialize = "file")]
    File,
    #[strum(serialize = "git+https")]
    GitHttps,
    #[strum(serialize = "https")]
    Https,
    #[strum(serialize = "http")]
    Http,
}

#[derive(Debug)]
pub(crate) struct Url {
    url: url::Url,
    scheme: Scheme,
}

impl Url {
    pub(crate) fn parse_with_quirks_mode(str: &str, current_dir: Option<PathBuf>) -> Result<Self> {
        if let Err(url::ParseError::RelativeUrlWithoutBase) = url::Url::parse(str) {
            let path = if let Some(current_dir) = current_dir {
                current_dir.join(str)
            } else {
                env::current_dir().map_err(IoError)?.join(str)
            };
            let str = path
                .to_str()
                .ok_or(str)
                .map_err(|e| Error::UrlParseError(InvalidFilePath(e.to_string())))?;
            Self::parse(&format!("file://{}", str))
        } else {
            Self::parse(str)
        }
    }

    fn parse(str: &str) -> Result<Self> {
        let url = url::Url::parse(str).map_err(UrlParseError)?;
        let scheme = Self::parse_scheme(url.scheme())?;
        Ok(Self { url, scheme })
    }

    fn parse_scheme(str: &str) -> Result<Scheme> {
        Scheme::from_str(str).map_err(|_| {
            let v = Vec::from_iter(Scheme::iter().map(|v| v.to_string()));
            let e = format!("the scheme was expected one of {:?} but got {:?}", v, str);
            Error::UrlParseError(UnknownScheme(e))
        })
    }

    pub fn scheme(&self) -> Scheme {
        self.scheme.clone()
    }

    delegate::delegate! {
        to self.url {
            pub fn fragment(&self) -> Option<&str>;
            pub fn to_file_path(&self) -> std::result::Result<PathBuf, ()>;
        }
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}
