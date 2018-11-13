FROM rustlang/rust:nightly as build

ARG OUT_DIR=./target/docker/
ARG CARGO_FLAGS

RUN USER=root cargo new hato
WORKDIR /hato
ADD ./Cargo.lock ./Cargo.lock
ADD ./Cargo.toml ./Cargo.toml
RUN cargo build $CARGO_FLAGS

ADD . /hato
RUN touch src/* && \
    cargo build -Z unstable-options --out-dir $OUT_DIR $CARGO_FLAGS

FROM debian:stretch-slim

ARG OUT_DIR=./target/docker/
COPY --from=build /hato/$OUT_DIR/hato /

EXPOSE 8080

CMD ["/hato"]
