use thirtyfour::prelude::*;

enum Url {
    Base,
    Search,
    Case,
}

struct Scraper {
    driver: Webdriver,
}

impl Scraper {
    pub fn new(&self, driver: Webdriver) -> Self {
        self.driver = driver;
    }
}
