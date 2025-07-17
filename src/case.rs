use scraper::{Html, Selector};
// use std::error::Error;

#[derive(Debug, Clone)]
pub struct Case {
    pub name: String,
    pub docket: String,
    pub defendant: String,
    pub property_address: String,
}
pub struct Cases {
    pub cases: Vec<Case>,
}

impl Cases {
    pub fn new() -> Self {
        Cases { cases: Vec::new() }
    }

    pub fn extract(&mut self, html: &str) {
        let doc = Html::parse_document(&html);

        // Selector for table rows inside the result table
        let row_selector =
            Selector::parse(r#"table[id="ctl00_ContentPlaceHolder1_gvPropertyResults"] tr"#)
                .unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let a_selector = Selector::parse("a").unwrap();

        for row in doc.select(&row_selector) {
            let defendant = "".to_string();
            let property_address = "".to_string();
            let tds: Vec<_> = row.select(&td_selector).collect();

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
}
