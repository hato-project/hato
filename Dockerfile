FROM rustlang/rust:nightly as build

ARG OUT_DIR=./target/docker/
ARG CARGO_FLAGS

RUN USER=root cargo new --bin hato
WORKDIR /hato
ADD ./Cargo.lock ./Cargo.lock
ADD ./Cargo.toml ./Cargo.toml
RUN cargo build -Z unstable-options --out-dir $OUT_DIR $CARGO_FLAGS

ADD . /hato
RUN cargo build -Z unstable-options --out-dir $OUT_DIR $CARGO_FLAGS

FROM rustlang/rust:nightly

ARG OUT_DIR=./target/docker/
COPY --from=build /hato/$OUT_DIR/hato .

EXPOSE 8000

CMD ["./hato"]