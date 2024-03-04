use chrono::NaiveDate;
use diesel::{insert_into, RunQueryDsl};
use reqwest::blocking::Client;
use reqwest::Error;
use scraper::Selector;
use uuid::Uuid;
use web_scraper::*;
use models::EtlRawSummary;

fn get_website_content(url: &str) -> Result<String, Error> {
    let client: Client = Client::builder()
        // .proxy(
        //     reqwest::Proxy::http("xxxx")?
        //         .no_proxy(reqwest::NoProxy::from_string("localhost,xxxx")),
        // )
        // .proxy(reqwest::Proxy::https("xxxx")?)
        .build()?;

    let response: String = client.get(url).send()?.text()?;
    Ok(response)
}

fn scrap_summary_table(content: &str) -> Result<Vec<EtlRawSummary>, Error> {
    let document = scraper::Html::parse_document(content);

    let page_content_selector: Selector = Selector::parse("article.page-content__item").unwrap(); // unwrapping just the selector creating
    let table_content_selector: Selector = Selector::parse("tbody.table__tbody").unwrap(); // unwrapping just the selector creating
    let table_entry_selector: Selector = Selector::parse("tr.table__tr").unwrap(); // unwrapping just the selector creating
    let table_data_selector: Selector = Selector::parse("td.table__td").unwrap();
    let a_selector: Selector = Selector::parse("a").unwrap();

    let mut result = vec![];
    for page_content in document.select(&page_content_selector) {
        for table_content in page_content.select(&table_content_selector) {
            for entry in table_content.select(&table_entry_selector) {
                let mut table_data = entry.select(&table_data_selector);
                if table_data.clone().count() == 5 {
                    let date_str = table_data.next().unwrap().text().collect::<String>(); // can't be empty
                    let date = NaiveDate::parse_from_str(&date_str, "%d.%m.%y").unwrap();

                    let company_element = table_data.next().unwrap();
                    let company_name = company_element.text().collect::<String>();
                    let company_link = company_element
                        .select(&a_selector)
                        .next()
                        .and_then(|a| a.value().attr("href").map(String::from));

                    let reporter_element = table_data.next().unwrap();
                    let reporter_name = reporter_element.text().collect::<String>();
                    let reporter_link = reporter_element
                        .select(&a_selector)
                        .next()
                        .and_then(|a| a.value().attr("href").map(String::from));

                    let row: EtlRawSummary = EtlRawSummary {
                        id: Uuid::new_v4(),
                        date: date,
                        company_name: company_name,
                        company_link: company_link,
                        reporter_name: reporter_name,
                        reporter_link: reporter_link,
                    };
                    result.push(row);
                }
            }
        }
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url: &str = "https://www.finanzen.net";
    let date: &str = "01.12.2023";
    let url: String = format!("{}/insiderdaten/?date={}", base_url, date);

    let response = get_website_content(url.as_str())?;
    let summary_table = scrap_summary_table(&response)?;
    println!("{summary_table:#?}");

    let connection: &mut diesel::prelude::PgConnection = &mut establish_connection();

    let _res = insert_into(crate::schema::etl_raw_summary::table)
    .values(summary_table).execute(connection);

    Ok(())
}