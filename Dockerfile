FROM rust:1.71.1 AS builder

COPY . .

RUN cargo build --release



FROM debian:buster-slim
EXPOSE 3003

COPY --from=builder /target/release/rust-tests /target/release/rust-tests

CMD ["/target/release/rust-tests"]