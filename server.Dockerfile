FROM rust:1.69 as builder
WORKDIR /usr/src/noughts-crosses-grpc-rust
RUN apt update && apt install -y protobuf-compiler libprotobuf-dev
COPY . .
RUN cargo install --path . --bin server

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
CMD ["server"]