use std::io::stdin;
use web_scraper::*;

fn main() {
    let connection: &mut diesel::prelude::PgConnection = &mut establish_connection();

    let mut company_name: String = String::new();
    let mut reporter_name: String = String::new();

    println!("Enter company name:");
    stdin().read_line(&mut company_name).unwrap();
    let company_name = company_name.trim_end(); // Remove the trailing newline

    println!("Enter reporter name:");
    stdin().read_line(&mut reporter_name).unwrap();
    let reporter_name = reporter_name.trim_end(); // Remove the trailing newline

    let etl_summary_entry: models::EtlRawSummary = create_etl_raw_summary(
        connection,
        chrono::Local::now().naive_local().date(),
        company_name.to_string(),
        None,
        reporter_name.to_string(),
        None,
    );

    println!(
        "\nSaved draft {} with id {}",
        etl_summary_entry.company_name, etl_summary_entry.reporter_name
    );
}
