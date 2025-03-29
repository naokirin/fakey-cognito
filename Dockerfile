FROM rust:1.84-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
     python3-dev \
  && apt-get -y clean \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/app/target/release/fakey_cognito"]
