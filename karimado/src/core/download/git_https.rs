use anyhow::Result;
use fastrand::alphanumeric;
use git2::Repository;
use std::{
    iter,
    path::{Path, PathBuf},
};
use url::Url;

pub(crate) fn download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    let repo_url = String::from(url.clone());
    let repo_url = repo_url.replace("git+https://", "https://");

    let repo_name: String = iter::repeat_with(alphanumeric).take(8).collect();
    let repo_path = downloads_path.join(repo_name);
    let repo = Repository::clone(&repo_url, &repo_path)?;

    if let Some(committish) = url.fragment() {
        let (object, reference) = repo.revparse_ext(committish)?;
        repo.checkout_tree(&object, None)?;

        match reference {
            Some(r) => repo.set_head(r.name().unwrap()),
            None => repo.set_head_detached(object.id()),
        }?;
    }

    Ok(repo_path)
}
