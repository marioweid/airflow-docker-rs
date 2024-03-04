FROM rust:slim-buster

# ENV https_proxy=xxx
# ENV http_proxy=xxx
# ENV HTTPS_PROXY=xxx
# ENV HTTP_PROXY=xxx
# ENV NO_PROXY="xxx"

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev libssl-dev pkg-config


WORKDIR /app
COPY . .

RUN cargo build

CMD ["echo", "Hello from Rust Dockerfile!"]