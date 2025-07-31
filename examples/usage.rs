use foreclose_scrape::scraper::Scraper;
use std::error::Error;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Setup WebDriver
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Use the new fluent API to scrape foreclosure cases
    let cases = Scraper::new(driver)
        .search_by_town("Middletown")
        .extract_cases()
        .await?;

    // Print the results
    println!("Found {} cases:", cases.len());
    for case in &cases {
        println!(
            "Case: {} | Docket: {} | Defendant: {} | Property: {}",
            case.name, case.docket, case.defendant, case.property_address
        );
    }

    Ok(())
}
