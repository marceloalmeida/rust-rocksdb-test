FROM rust:1.36.0-stretch AS builder

RUN apt-get update && apt-get -yq install libclang-dev clang-4.0

WORKDIR /app

ADD . /app

RUN cd /app && cargo build --release -vv


FROM debian:stretch-slim
WORKDIR /app
COPY --from=builder /app/target/release/main .
VOLUME "/srv/rocksdb"
CMD ["/app/main"] 
