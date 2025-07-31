use crate::case::Cases;
use thirtyfour::prelude::*;

enum Judicial {
    Base,
    SearchPage,
    CaseDetail,
}

impl Judicial {
    pub fn as_str(&self) -> &'static str {
        match self {
            Judicial::Base => "https://civilinquiry.jud.ct.gov",
            Judicial::SearchPage => "https://civilinquiry.jud.ct.gov/PropertyAddressSearch.aspx",
            Judicial::CaseDetail => {
                "https://civilinquiry.jud.ct.gov/CaseDetail/PublicCaseDetail.aspx?DocketNo="
            }
        }
    }
}

pub struct Jscraper {
    driver: WebDriver,
    cases: Cases,
}

impl Jscraper {
    pub fn new(driver: WebDriver) -> Self {
        let cases = Cases::new();
        Self { driver, cases }
    }

    async fn get_html_page(&self, town: &str) -> Result<String, WebDriverError> {
        let site = Judicial::SearchPage.as_str();
        self.driver.goto(site).await?;
        self.driver
            .find(By::Id("ctl00_ContentPlaceHolder1_txtCityTown"))
            .await?
            .send_keys(town)
            .await?;

        // Click search button
        self.driver
            .find(By::Id("ctl00_ContentPlaceHolder1_btnSubmit"))
            .await?
            .click()
            .await?;

        // Wait for the table to appear (poll until timeout)
        let table_id = "ctl00_ContentPlaceHolder1_gvPropertyResults";
        self.driver.query(By::Id(table_id)).first().await?; // this waits for the element to be present

        // Get updated page HTML
        let html = self.driver.source().await?;
        Ok(html)
    }

    pub async fn get_cases(&self, html: String) -> Result<Self, WebDriverError> {
        // Extract the result table
        if let Some(html) = get_html_page(&html, table_id) {
            tokio::fs::write("./output/foreclose-cases.html", &table_html).await?;
            cases.extract(&table_html);
        } else {
            println!("Table not found.");

    }
}
