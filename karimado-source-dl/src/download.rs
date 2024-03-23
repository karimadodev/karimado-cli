#[derive(Default)]
pub struct Download {
    pub(crate) url: String,
    pub(crate) dirname: Option<String>,
}

impl Download {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn dirname(&mut self, dirname: &str) -> &mut Self {
        self.dirname = Some(dirname.to_string());
        self
    }
}
