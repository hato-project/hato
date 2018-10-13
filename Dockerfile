FROM rustlang/rust:nightly as build

RUN USER=root cargo new --bin hato
WORKDIR /hato

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release

FROM rustlang/rust:nightly

COPY --from=build /hato/target/release/hato .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["./hato"]