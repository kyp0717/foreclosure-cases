pub enum Judiciary {
    Base(String),
    Town(String),
}

enum UrlError {
    IncorrectBase(String),
}

impl Judiciary {
    fn base_url(&self) -> Result<&str, UrlError> {
        let site = "https://civilinquiry.jud.ct.gov/PropertyAddressSearch.aspx";
        match self {
            &Judiciary::Base(ref site) => Ok(site),
            _ => Err(UrlError::IncorrectBase("wrong base url".to_string())),
        }
    }
}
