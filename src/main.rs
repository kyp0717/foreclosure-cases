use std::error::Error;
use thirtyfour::prelude::*;
use scraper::{Html, Selector};
// use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::chrome();
    // caps.add_arg("--headless=new")?; // enable in headless mode

    let driver = WebDriver::new("http://localhost:35193", caps).await?;
    let site = "https://civilinquiry.jud.ct.gov/PropertyAddressSearch.aspx";
    driver.goto(site).await?;

    // Enter city name
    driver
        .find(By::Id("ctl00_ContentPlaceHolder1_txtCityTown"))
        .await?
        .send_keys("Middletown")
        .await?;

    // Click search button
    driver
        .find(By::Id("ctl00_ContentPlaceHolder1_btnSubmit"))
        .await?
        .click()
        .await?;

    // Wait for the table to appear (poll until timeout)
    let table_id = "ctl00_ContentPlaceHolder1_gvPropertyResults";
    driver
        .query(By::Id(table_id))
        .first()
        .await?; // this waits for the element to be present

    // Get updated page HTML
    let html = driver.source().await?;

    // Save full HTML for inspection
    tokio::fs::write("page.html", &html).await?;

    // Extract the result table
    if let Some(table_html) = extract_table(&html, table_id) {
        tokio::fs::write("case_table.html", table_html).await?;
    } else {
        println!("Table not found.");
    }

    driver.quit().await?;
    Ok(())
}

fn extract_table(html: &str, tid: &str) -> Option<String> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse(&format!(r#"table[id="{}"]"#, tid)).ok()?;
    let table_element = doc.select(&selector).next()?;
    Some(table_element.html())
}
