use anyhow::Result;
use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};

#[derive(Default)]
pub(crate) struct Options {
    globs_include: Vec<Glob>,
}

impl Options {
    pub(crate) fn new() -> Self {
        Default::default()
    }

    pub(crate) fn add_include(&mut self, glob: &str) -> Result<()> {
        self.globs_include
            .push(GlobBuilder::new(glob).literal_separator(true).build()?);
        Ok(())
    }

    pub(crate) fn globset_include(&self) -> Result<GlobSet> {
        let mut builder = GlobSetBuilder::new();
        for glob in &self.globs_include {
            builder.add(glob.clone());
        }
        Ok(builder.build()?)
    }
}
