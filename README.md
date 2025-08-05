# Foreclosure Cases Scraper

A Rust application for scraping foreclosure case information from Connecticut's judicial inquiry system.

## Features

- Clean, fluent API for scraping foreclosure cases
- Extracts detailed case information including defendant names and property addresses
- Exports results to CSV format
- Built with async/await support using Tokio

## Usage

The scraper provides a simple, chainable API:

```rust
use foreclose_scrape::scraper::Scraper;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Setup WebDriver
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Scrape cases using the fluent API
    let cases = Scraper::new(driver)
        .search_by_town("Middletown")
        .extract_cases()
        .await?;

    // Process results
    println!("Found {} cases", cases.len());
    for case in &cases {
        println!("{}: {} - {}", case.docket, case.defendant, case.property_address);
    }

    Ok(())
}
```

## API Reference

### `Scraper::new(driver: WebDriver) -> Scraper`

Creates a new scraper instance with the provided WebDriver.

### `scraper.search_by_town(town: &str) -> SearchBuilder`

Initiates a search for the specified town/city.

### `search_builder.extract_cases() -> Result<Vec<Case>, Error>`

Executes the search and extracts all case information, returning a vector of `Case` structs.

## Case Structure

Each `Case` contains:

- `name`: Case name/title
- `docket`: Docket number
- `defendant`: Defendant name (extracted from case details)
- `property_address`: Property address (extracted from case details)

## Requirements

- Rust 1.70+
- Chrome/Chromium browser
- ChromeDriver running on localhost (default port 9515)

## Setup

1. Install ChromeDriver and ensure it's running:
   ```bash
   chromedriver --port=9515
   ```

2. Run the application:
   ```bash
   cargo run
   ```

The application will scrape cases for "Middletown" by default and save results to `./output/cases.csv`.

## Dependencies

- `thirtyfour` - WebDriver client for browser automation
- `scraper` - HTML parsing and CSS selector support
- `tokio` - Async runtime
- `csv` - CSV file generation
- `reqwest` - HTTP client (with various features)

## Output

Results are saved to:
- `./output/cases.csv` - CSV file with all case data
- `./output/page.html` - Full search results page (for debugging)
- `./output/case_*.html` - Individual case detail pages (for debugging)