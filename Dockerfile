FROM rust:1.55-slim

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/app/target/release/fakey-cognito"]