FROM rust:latest

WORKDIR /usr/src/myproject
COPY . .

RUN cargo build --release
EXPOSE 8080

CMD ["./target/release/http_server"]


