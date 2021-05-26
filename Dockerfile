ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

ADD --chown=rust:rust . ./

RUN cargo build --release

ENV DATABASE_URL="sqlite://rshorty.sqlite"
RUN cargo install sqlx-cli && sqlx database create

FROM alpine:latest
VOLUME /home/rust/data
ENV DATABASE_URL="sqlite:///home/rust/data/rshorty.sqlite"
COPY --from=builder \
    /home/rust/src/rshorty.sqlite* /home/rust/data/

COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rshorty \
    /usr/local/bin/
CMD /usr/local/bin/rshorty
