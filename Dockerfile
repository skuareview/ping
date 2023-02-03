FROM debian:latest

WORKDIR /app

ADD src src
COPY Cargo.toml .

# Dependencies
RUN apt-get -y update
RUN apt-get -y install curl build-essential libpq-dev pkg-config netcat libssl-dev

# Rustup install (not yet packaged on Ubuntu)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Rust toolchain
RUN ${HOME}/.cargo/bin/rustup install 1.63.0

# Cargo stuff
RUN ${HOME}/.cargo/bin/cargo install cargo-watch
RUN ${HOME}/.cargo/bin/cargo install diesel_cli --no-default-features --features postgres
RUN ${HOME}/.cargo/bin/cargo build
