mod case;
mod scraper;

use case::save_cases_to_csv;
use scraper::Scraper;
use std::error::Error;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::chrome();
    // caps.add_arg("--headless=new")?; // enable in headless mode
    let port = "45119";
    let driver_path = format!("http://localhost:{}", port);

    let driver = WebDriver::new(driver_path, caps).await?;

    // Use the new fluent API
    let cases = Scraper::new(driver)
        .search_by_town("Middletown")
        .extract_cases()
        .await?;

    // Save results to CSV
    if let Err(e) = save_cases_to_csv(&cases, "./output/cases.csv") {
        eprintln!("Error saving cases to CSV: {}", e);
    } else {
        println!("Cases saved to ./output/cases.csv");
    }

    // Print the cases for verification
    for case in &cases {
        println!(
            "Name: {}, Docket: {}, Defendant: {}, Property Address: {}",
            case.name, case.docket, case.defendant, case.property_address
        );
    }

    Ok(())
}
