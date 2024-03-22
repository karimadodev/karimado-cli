#[cfg(test)]
#[path = "reqwest_test.rs"]
mod tests;

use reqwest::blocking::{Client, Response};
use reqwest::Proxy;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use crate::{archive, contrib, error::*, Url};
use SourceDownloadErrorKind::{IoError, ReqwestError};

pub(crate) fn download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    let archive_url = url.to_string();
    let mut response = http_get(&archive_url)?;

    let archive_name: String = contrib::uuid();
    let archive_path = downloads_path.join(archive_name.clone() + ".download");
    let mut file = File::create(&archive_path).map_err(IoError)?;
    io::copy(&mut response, &mut file).map_err(IoError)?;

    let target_path = downloads_path.join(archive_name);
    archive::decompress(&archive_path, &target_path)?;
    fs::remove_file(&archive_path).map_err(IoError)?;

    Ok(target_path)
}

fn http_get(url: &str) -> Result<Response> {
    let mut builder = Client::builder();
    builder = builder.no_proxy();

    let proxies = builder_proxies()?;
    for proxy in proxies {
        builder = builder.proxy(proxy);
    }

    let client = builder.build().map_err(ReqwestError)?;
    let response = client.get(url).send().map_err(ReqwestError)?;
    Ok(response)
}

fn builder_proxies() -> Result<Vec<Proxy>> {
    let mut proxies: Vec<Proxy> = vec![];

    for var in ["HTTPS_PROXY", "ALL_PROXY"] {
        if let Ok(url) = env::var(var) {
            log::debug!("set up https proxy using env {}={}", var, url);
            proxies.push(Proxy::https(url).map_err(ReqwestError)?);
        }
    }
    for var in ["HTTP_PROXY", "ALL_PROXY"] {
        if let Ok(url) = env::var(var) {
            log::debug!("set up http proxy using env {}={}", var, url);
            proxies.push(Proxy::http(url).map_err(ReqwestError)?);
        }
    }

    Ok(proxies)
}
