FROM rust:1.61.0
WORKDIR /usr/src/wstat
COPY . .
RUN cargo install --path .

RUN cargo build --release

CMD ["./target/release/wstat"]