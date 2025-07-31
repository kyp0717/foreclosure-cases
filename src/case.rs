use csv::Writer;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Case {
    pub name: String,
    pub docket: String,
    pub defendant: String,
    pub property_address: String,
}

impl Case {
    pub fn new(name: String, docket: String) -> Self {
        Self {
            name,
            docket,
            defendant: String::new(),
            property_address: String::new(),
        }
    }

    pub fn to_csv_record(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.docket.clone(),
            self.defendant.clone(),
            self.property_address.clone(),
        ]
    }
}

pub fn save_cases_to_csv(cases: &[Case], filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut wtr = Writer::from_writer(file);

            // Each valid row should have at least 5 columns
            if tds.len() >= 5 {
                let name = tds[3].text().collect::<String>().trim().to_string();
                let docket_link = tds[4].select(&a_selector).next();

                if let Some(link) = docket_link {
                    let docket = link.text().collect::<String>().trim().to_string();
                    self.cases.push(Case {
                        name,
                        docket,
                        defendant,
                        property_address,
                    });
                }
            }
        }
    }

    pub fn get_defendant(doc: &Html) -> Option<String> {
        let selector = Selector::parse(
            r#"span#ctl00_ContentPlaceHolder1_CaseDetailParties1_gvParties_ctl05_lblPtyPartyName"#,
        )
        .ok()?;
        doc.select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
    }

    pub fn get_property_address(doc: &Html) -> Option<String> {
        let selector = Selector::parse(
            r#"span#ctl00_ContentPlaceHolder1_CaseDetailBasicInfo1_lblPropertyAddress"#,
        )
        .ok()?;
        doc.select(&selector)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
    }

    pub fn get_html_table(html: &str, tid: &str) -> Option<String> {
        let doc = Html::parse_document(html);
        let selector = Selector::parse(&format!(r#"table[id="{}"]"#, tid)).ok()?;
        let table_element = doc.select(&selector).next()?;
        Some(table_element.html())
    }

    pub fn save_to_csv(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = File::create(filename)?;
        let mut wtr = Writer::from_writer(file);

        // Write header
        wtr.write_record(&["Name", "Docket", "Defendant", "Property Address"])?;

        // Write case records
        for case in &self.cases {
            wtr.write_record(&case.to_csv_record())?;
        }

        wtr.flush()?;
        Ok(())
    }
}
