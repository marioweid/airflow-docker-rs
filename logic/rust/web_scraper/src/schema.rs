// @generated automatically by Diesel CLI.

diesel::table! {
    etl_raw_summary (id) {
        id -> Uuid,
        date -> Date,
        company_name -> Text,
        company_link -> Nullable<Text>,
        reporter_name -> Text,
        reporter_link -> Nullable<Text>,
    }
}