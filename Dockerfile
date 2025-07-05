FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release


FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates
COPY --from=builder /app/target/release/Wally /usr/local/bin/app

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/app"]
