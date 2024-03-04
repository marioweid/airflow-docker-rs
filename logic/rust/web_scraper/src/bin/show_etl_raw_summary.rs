use self::models::*;
use diesel::prelude::*;
use web_scraper::*;

fn main() {
    use self::schema::etl_raw_summary::dsl::*;

    let connection = &mut establish_connection();
    let results:Vec<EtlRawSummary> = etl_raw_summary
        .limit(5)
        .select(EtlRawSummary::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for row in results {
        println!("{}", row.id);
        println!("{}", row.date);
        println!("{}", row.company_name);
        println!("{:?}", row.company_link);
        println!("{}", row.reporter_name);
        println!("{:?}", row.company_link);
        println!("-----------\n");
    }
}