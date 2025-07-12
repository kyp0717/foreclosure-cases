use scraper::{Html, Selector};
// pub struct Person {
//     first: String,
//     last: String,
//     phone: String,
//     email: String,
// }

pub struct Case {
    pub name: String,
    pub docket: String,
    // pub defendant: Vec<Person>,
}


impl Case {
    fn new(
        name: String,
        docket: String
    ) -> Self {
        Case {
            name,
            docket
        }
    }
    pub fn from_html(html: &str) -> Vec<Case> {
        let mut cases = Vec::new();
        let doc = Html::parse_document(html);
        let selector = Selector::parse("table#ctl00_ContentPlaceHolder1_gvCases > tbody > tr").unwrap();

        for row in doc.select(&selector) {
            let cells: Vec<_> = row.select(&Selector::parse("td").unwrap()).collect();
            if cells.len() < 3 {
                continue; // Skip rows that don't have enough data
            }

            let name = cells[0].text().collect::<Vec<_>>().join("").trim().to_string();
            let docket = cells[1].text().collect::<Vec<_>>().join("").trim().to_string();

            cases.push(Case::new(name, docket));
        }
        cases
    }

}

