pub enum UrlError {
    IncorrectBase(String),
}
pub trait UrlBuilder {
    fn base_url(&self) -> Result<&str, UrlError>;
    fn path(&self) -> String;
    fn query(&self) -> Option<String> {
        None
    }
    fn fragment(&self) -> Option<String> {
        None
    }

    fn build_url(&self) -> String {
        let mut url = format!("{}/{}", self.base_url().trim_end_matches('/'), self.path());

        if let Some(q) = self.query() {
            url.push('?');
            url.push_str(&q);
        }

        if let Some(f) = self.fragment() {
            url.push('#');
            url.push_str(&f);
        }

        url
    }
}
