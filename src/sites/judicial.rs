use super::builder::{UrlBuilder, UrlError};

pub enum Judiciary {
    Base,
    Town(String),
}

impl UrlBuilder for Judiciary {
    fn base_url(&self) -> Result<&str, UrlError> {
        let site = "https://civilinquiry.jud.ct.gov/PropertyAddressSearch.aspx";
        match self {
            &Judiciary::Base => Ok(site),
            _ => Err(UrlError::IncorrectBase("wrong url".to_string())),
        }
    }
    fn path(&self) -> String {
        match self {
            Judiciary::Town { town } => format!("products/{}", id),
            ExampleSite::Search { .. } => "search".to_string(),
            ExampleSite::Docs { topic } => format!("{}/index.html", topic),
        }
    }

    fn query(&self) -> Option<String> {
        match self {
            ExampleSite::Search { query, category } => {
                let mut parts = vec![format!("q={}", urlencoding::encode(query))];
                if let Some(cat) = category {
                    parts.push(format!("category={}", urlencoding::encode(cat)));
                }
                Some(parts.join("&"))
            }
            _ => None,
        }
    }
}
