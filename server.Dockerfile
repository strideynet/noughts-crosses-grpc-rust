FROM rust:1.69 as builder

COPY ./ ./

RUN cargo build --release --bin server

FROM debian:buster-slim

COPY --from=builder ./target/release/server ./server
CMD ["./server"]