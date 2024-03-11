use anyhow::Result;
use fastrand::alphanumeric;
use git2::{build::RepoBuilder, FetchOptions, ProxyOptions, Repository};
use std::{
    env, iter,
    path::{Path, PathBuf},
};
use url::Url;

pub(crate) fn download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    let repo_url = url.clone().to_string();
    let repo_url = repo_url.replace("git+https://", "https://");

    let repo_name: String = iter::repeat_with(alphanumeric).take(8).collect();
    let repo_path = downloads_path.join(repo_name);
    let repo = git_clone_repoistory(&repo_url, &repo_path)?;

    if let Some(committish) = url.fragment() {
        let (object, reference) = repo.revparse_ext(committish)?;
        repo.checkout_tree(&object, None)?;

        match reference {
            Some(r) => repo.set_head(r.name().expect("invalid reference name")),
            None => repo.set_head_detached(object.id()),
        }?;
    }

    Ok(repo_path)
}

fn git_clone_repoistory(repo_url: &str, repo_path: &Path) -> Result<Repository> {
    let mut builder = RepoBuilder::new();
    let mut builder = match env::var("HTTPS_PROXY") {
        Ok(value) => {
            log::debug!("set up proxy using env HTTPS_PROXY={}", value);

            let mut proxy_options = ProxyOptions::new();
            proxy_options.url(&value);

            let mut fetch_options = FetchOptions::new();
            fetch_options.proxy_options(proxy_options);

            builder.fetch_options(fetch_options);
            builder
        }
        _ => builder,
    };

    log::debug!("cloning new repository: {}", repo_url);
    Ok(builder.clone(repo_url, repo_path)?)
}
