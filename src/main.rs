use scraper::{Html, Selector};
use std::error::Error;
use thirtyfour::prelude::*;
// use std::time::Duration;

#[derive(Debug)]
struct Case {
    name: String,
    docket: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::chrome();
    // caps.add_arg("--headless=new")?; // enable in headless mode

    let driver = WebDriver::new("http://localhost:40319", caps).await?;
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
    driver.query(By::Id(table_id)).first().await?; // this waits for the element to be present

    // Get updated page HTML
    let html = driver.source().await?;

    // Save full HTML for inspection
    tokio::fs::write("page.html", &html).await?;

    // Extract the result table
    if let Some(table_html) = extract_table(&html, table_id) {
        tokio::fs::write("case_table.html", &table_html).await?;
        let cases = extract_case(&table_html);
        for case in &cases {
            println!("{:?}", case);
        }
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

fn extract_case(html: &str) -> Vec<Case> {
    let doc = Html::parse_document(&html);

    // Selector for table rows inside the result table
    let row_selector =
        Selector::parse(r#"table[id="ctl00_ContentPlaceHolder1_gvPropertyResults"] tr"#).unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let mut results = Vec::new();

    for row in doc.select(&row_selector) {
        let tds: Vec<_> = row.select(&td_selector).collect();

        // Each valid row should have at least 5 columns
        if tds.len() >= 5 {
            let name = tds[3].text().collect::<String>().trim().to_string();
            let docket_link = tds[4].select(&a_selector).next();

            if let Some(link) = docket_link {
                let docket = link.text().collect::<String>().trim().to_string();

                results.push(Case { name, docket });
            }
        }
    }

    results
}

fn save_cases_to_csv(cases: &[Case], path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(&["Case Name", "Docket Number"])?;

    for case in cases {
        wtr.write_record(&[&case.name, &case.docket])?;
    }

    wtr.flush()?;
    Ok(())
}
