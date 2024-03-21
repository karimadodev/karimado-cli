mod scheme;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use strum::IntoEnumIterator;

use crate::error::*;
use UrlParseErrorKind::{UrlParseError, UnknownScheme};

pub(crate) use scheme::*;

pub(crate) struct Url {
    url: url::Url,
    scheme: Scheme,
}

impl Url {
    pub(crate) fn parse(input: &str) -> Result<Self> {
        let url = url::Url::parse(input).map_err(UrlParseError)?;
        let scheme = Self::parse_scheme(url.scheme())?;
        Ok(Self { url, scheme })
    }

    fn parse_scheme(scheme: &str) -> Result<Scheme> {
        if let Ok(scheme) = Scheme::from_str(scheme) {
            Ok(scheme)
        } else {
            let v = Vec::from_iter(Scheme::iter().map(|v| v.to_string()));
            let e = format!("the scheme was expected one of {:?} but got {:?}", v, scheme);
            Err(UnknownScheme(e))?
        }
    }

    pub fn scheme(&self) -> Scheme {
        self.scheme.clone()
    }

    pub fn fragment(&self) -> Option<&str> {
        self.url.fragment()
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}
