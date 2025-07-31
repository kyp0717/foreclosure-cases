mod case;
mod jdscraper;

use case::Cases;
use jdscraper::Jscraper;
use scraper::Html;
use std::error::Error;
use thirtyfour::prelude::*;
// use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::chrome();
    // caps.add_arg("--headless=new")?; // enable in headless mode
    let port = "45119";
    let driver_path = format!("http://localhost:{}", port);

    let driver = WebDriver::new(driver_path, caps).await?;

    // let judicial = Jscraper::new(driver);
    let site = "https://civilinquiry.jud.ct.gov/PropertyAddressSearch.aspx";
    let site_case = "https://civilinquiry.jud.ct.gov/CaseDetail/PublicCaseDetail.aspx?DocketNo=";

    let judicial = Jscraper::new(driver);

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
    driver.query(By::Id(table_id)).first().await?; // this waits for the element to be present

    // Get updated page HTML
    let html = driver.source().await?;

    // Save full HTML for inspection
    tokio::fs::write("./output/page.html", &html).await?;

    let mut cases = Cases::new();

    // Extract the result table
    if let Some(table_html) = Cases::get_html_table(&html, table_id) {
        tokio::fs::write("./output/foreclose-cases.html", &table_html).await?;
        cases.extract(&table_html);
    } else {
        println!("Table not found.");
    }

    for (i, case) in cases.cases.iter_mut().enumerate() {
        // remove the dashes in the docket number
        let docket_cleaned = case.docket.replace("-", "");
        // Construct the full case URL
        let case_url = format!("{}{}", site_case, docket_cleaned);
        // go to the site for each case
        driver.goto(&case_url).await?;
        // Wait for the page to load
        driver.query(By::Id("ctl00_tblContent")).first().await?;
        // Get the case details page HTML
        let case_html = driver.source().await?;
        // Save the case HTML for inspection, indexed by i
        let case_file_name = format!("./output/case_{:03}_{}.html", i + 1, docket_cleaned);
        tokio::fs::write(case_file_name, &case_html).await?;
        //extract the name and property address from the case HTML
        let doc = Html::parse_document(&case_html);
        // let property_address = extract_property_address(&doc).unwrap_or_default();
        if let Some(name) = Cases::get_defendant(&doc) {
            case.defendant = name;
        }

        if let Some(property) = Cases::get_property_address(&doc) {
            case.property_address = property;
        }
        if i == 5 {
            break;
        }
    }
    if let Err(e) = cases.save_to_csv("./output/cases.csv") {
        eprintln!("Error saving cases to CSV: {}", e);
    } else {
        println!("Cases saved to ./output/cases.csv");
    }

    // Print the cases for verification
    for case in &cases.cases {
        println!(
            "Name: {}, Docket: {}, Defendant: {}, Property Address: {}",
            case.name, case.docket, case.defendant, case.property_address
        );
    }
    driver.quit().await?;
    Ok(())
}
