pub struct Download {
    pub(crate) url: String,
}

impl Download {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
}
