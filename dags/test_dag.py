from datetime import datetime, timedelta
from airflow import DAG

from airflow.operators.bash_operator import BashOperator
from airflow.operators.docker_operator import DockerOperator
from docker.types import Mount

default_args = {
    "owner": "airflow",
    "description": "Use of the DockerOperator",
    "depend_on_past": False,
    "start_date": datetime(2021, 5, 1),
    "email_on_failure": False,
    "email_on_retry": False,
    "retries": 1,
    "retry_delay": timedelta(minutes=5),
}

with DAG(
    "docker_operator_dag",
    default_args=default_args,
    schedule_interval="5 * * * *",
    catchup=False,
) as dag:
    t1 = BashOperator(task_id="print_current_date", bash_command="date")

    t2 = DockerOperator(
        task_id="test_job_2",
        image="rust_webscraper",
        container_name="rust_webscraper",
        api_version="auto",
        auto_remove=True,
        command="cargo run --package web_scraper --bin scraper",
        docker_url="unix://var/run/docker.sock",
        network_mode="host",
        environment={
            "DATABASE_URL": "postgres://postgres:postgres@localhost:5437/test_db",
            # "HTTP_PROXY": "xxxx",
            # "HTTPS_PROXY": "xxxx",
            # "http_proxy": "xxxx",
            # "https_proxy": "xxxx",
        },
        xcom_all=False,
    )

    t1 >> t2