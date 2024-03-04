# Airflow Docker Rs
An small airflow pipeline that starts a docker operator to scrape data from an website and write it inside a postgresql database.

## Links:
- [Airflow Docker Documentation](https://airflow.apache.org/docs/apache-airflow/stable/howto/docker-compose/index.html)
- [Diesel Documentation](https://diesel.rs/guides/getting-started)

## Run the App
### Build the Container
`docker build -t rust_webscraper -/logic/rust/web_scraper`
### Run airflow
`docker compose up -d`

### Install Cargo cli (for local development)
- [Setup cargo cli](https://stackoverflow.com/questions/70383711/problem-trying-to-install-diesel-mac-air-m1)
- `cargo build`
