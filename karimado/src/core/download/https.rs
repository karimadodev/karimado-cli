use anyhow::Result;
use fastrand::alphanumeric;
use std::{
    fs::{self, File},
    io, iter,
    path::{Path, PathBuf},
};
use url::Url;

use crate::core::archive;

pub(crate) fn download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    let archive_url = String::from(url.clone());
    let mut response = reqwest::blocking::get(archive_url)?;

    let archive_name: String = iter::repeat_with(alphanumeric).take(8).collect();
    let archive_path = downloads_path.join(archive_name.clone() + ".download");
    let mut file = File::create(&archive_path)?;
    io::copy(&mut response, &mut file)?;

    let target_path = downloads_path.join(archive_name);
    archive::decompress(&archive_path, &target_path)?;
    fs::remove_file(&archive_path)?;

    Ok(target_path)
}
