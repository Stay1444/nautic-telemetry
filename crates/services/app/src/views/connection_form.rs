pub struct ConnectionForm {
    url: String,
    error: Option<String>,
}

impl ConnectionForm {
    pub fn new(url: String) -> Self {
        Self { url, error: None }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_ref().map(|x| x.as_str())
    }
}
