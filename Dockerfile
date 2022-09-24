FROM rust:latest

WORKDIR /usr/src/rusty

COPY . .

RUN cargo build --release

CMD cargo run
