FROM rust:latest as build

WORKDIR /src/lgtm

RUN apt update && \
  apt install -y musl musl-tools && \
  rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

RUN USER=root cargo new --lib lgtm  && \
  cargo new --bin api && \
  cargo new --bin cli

RUN cargo fetch

COPY lgtm ./lgtm
COPY cli ./cli

RUN cargo build --target x86_64-unknown-linux-musl -p cli --release

FROM scratch as cli

COPY --from=build /src/lgtm/target/x86_64-unknown-linux-musl/release/cli /lgtm_cli

CMD ["/lgtm_cli"]
