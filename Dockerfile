FROM rust:1-buster as builder
WORKDIR /usr/src/app/
COPY Cargo.toml .
COPY src src
# test and build
RUN cargo test --release
RUN cargo build --release

FROM debian:buster
RUN apt-get update \
&& apt-get install \
-y \
--no-install-recommends \
pkg-config \
libssl-dev \
ca-certificates \
&& rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/simple_crawler_web .
CMD ["./simple_crawler_web"]


