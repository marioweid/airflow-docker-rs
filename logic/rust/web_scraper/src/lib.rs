pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::EtlRawSummary;

pub fn create_etl_raw_summary(
    conn: &mut PgConnection,
    date: chrono::NaiveDate,
    company_name: String,
    company_link: Option<String>,
    reporter_name: String,
    reporter_link: Option<String>,
) -> EtlRawSummary {
    use crate::schema::etl_raw_summary;

    let new_summary: EtlRawSummary = EtlRawSummary {
        id: uuid::Uuid::new_v4(),
        date: date,
        company_name: company_name,
        company_link: company_link,
        reporter_name: reporter_name,
        reporter_link: reporter_link,
    };

    diesel::insert_into(etl_raw_summary::table)
        .values(&new_summary)
        .returning(EtlRawSummary::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}