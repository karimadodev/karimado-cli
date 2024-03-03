use anyhow::Result;
use std::{fs::File, io::prelude::*, path::Path};

pub(crate) fn is_file_identical(path1: &Path, path2: &Path) -> Result<bool> {
    let mut buf1 = Vec::new();
    File::open(path1)?.read_to_end(&mut buf1)?;

    let mut buf2 = Vec::new();
    File::open(path2)?.read_to_end(&mut buf2)?;

    Ok(buf1 == buf2)
}
