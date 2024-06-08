FROM rust:buster as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/witch-qa-familiar .
EXPOSE 5000
CMD ["./witch-qa-familiar"]