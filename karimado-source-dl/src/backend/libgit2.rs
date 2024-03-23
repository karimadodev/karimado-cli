#[cfg(test)]
#[path = "libgit2_test.rs"]
mod tests;

use git2::{build::RepoBuilder, FetchOptions, ProxyOptions, Repository};
use std::env;
use std::path::{Path, PathBuf};

use crate::{contrib, error::*, Url};
use SourceDownloadErrorKind::Git2Error;

pub(crate) fn download(url: &Url, downloads_path: &Path) -> Result<PathBuf> {
    let repo_path = downloads_path.join(contrib::uuid());
    let repo = git_clone_repoistory(&url.to_string(), &repo_path)?;

    if let Some(committish) = url.fragment() {
        let (object, reference) = repo.revparse_ext(committish).map_err(Git2Error)?;
        repo.checkout_tree(&object, None).map_err(Git2Error)?;

        match reference {
            Some(r) => repo.set_head(r.name().expect("invalid reference name")),
            None => repo.set_head_detached(object.id()),
        }
        .map_err(Git2Error)?;
    }

    Ok(repo_path)
}

fn git_clone_repoistory(repo_url: &str, repo_path: &Path) -> Result<Repository> {
    let mut builder = RepoBuilder::new();

    let proxy_options = builder_proxy_options(repo_url);
    if let Some(proxy_options) = proxy_options {
        let mut fetch_options = FetchOptions::new();
        fetch_options.proxy_options(proxy_options);
        builder.fetch_options(fetch_options);
    }

    log::debug!("cloning new repository: {}", repo_url);
    Ok(builder.clone(repo_url, repo_path).map_err(Git2Error)?)
}

fn builder_proxy_options(repo_url: &str) -> Option<ProxyOptions> {
    let proxy_options_from_vars = |vars: &[&str]| -> Option<ProxyOptions> {
        for var in vars {
            if let Ok(url) = env::var(var) {
                log::debug!("set up http(s) proxy using env {}={}", var, url);
                let mut options = ProxyOptions::new();
                options.url(&url);
                return Some(options);
            }
        }
        None
    };

    if repo_url.starts_with("https://") {
        proxy_options_from_vars(&["HTTPS_PROXY", "ALL_PROXY"])
    } else if repo_url.starts_with("http://") {
        proxy_options_from_vars(&["HTTP_PROXY", "ALL_PROXY"])
    } else {
        unreachable!()
    }
}
