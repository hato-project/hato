FROM rustlang/rust:nightly as build

ARG OUT_DIR=./target/docker/
ARG CARGO_FLAGS

RUN USER=root cargo new hato
WORKDIR /hato
RUN mkdir -p src/server src/agent && echo "fn main() {}" | tee src/server/main.rs src/agent/main.rs
ADD ./Cargo.lock ./Cargo.lock
ADD ./Cargo.toml ./Cargo.toml
RUN cargo build --bins -Z unstable-options $CARGO_FLAGS

ADD . /hato
RUN touch src/* && \
    cargo build --bins -Z unstable-options --out-dir $OUT_DIR $CARGO_FLAGS

FROM debian:stretch-slim

RUN apt-get update && apt-get install -y libpq5

ARG OUT_DIR=./target/docker/
COPY --from=build /hato/$OUT_DIR/ /app

EXPOSE 8000

CMD ["/app/hato-server"]
