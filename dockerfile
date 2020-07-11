FROM rust:1.44-buster

WORKDIR /usr/src/app

RUN USER=root cargo init --bin \
  && apt-get update \
  && apt-get install -y gcc pkg-config openssl libasound2-dev cmake build-essential python3 libfreetype6-dev libexpat1-dev libxcb-composite0-dev libssl-dev libx11-dev


ENV CARGO_TARGET_DIR=/tmp/target

COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock

RUN cargo build \
  && rm src/main.rs

EXPOSE 8000
