mod options;
mod worker;

use anyhow::Result;
use std::path::Path;

pub(crate) type SyncOptions = options::Options;

pub(crate) fn sync(source: &Path, destination: &Path, options: &SyncOptions) -> Result<()> {
    let worker = worker::Worker::new(source, destination);
    worker.sync(options)
}
