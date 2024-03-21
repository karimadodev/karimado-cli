use std::path::{Path, PathBuf};

use crate::{error::*, Url};

pub(crate) fn download(_url: &Url, _downloads_path: &Path) -> Result<PathBuf> {
    Ok(PathBuf::new())
}
