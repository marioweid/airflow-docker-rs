// src/models.rs
use diesel::prelude::*;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::etl_raw_summary)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct EtlRawSummary {
    pub id: Uuid,
    pub date: NaiveDate,
    pub company_name: String,
    pub company_link: Option<String>,
    pub reporter_name: String,
    pub reporter_link: Option<String>,
}