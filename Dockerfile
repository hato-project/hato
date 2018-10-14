FROM rust:1 as build

WORKDIR /hato

COPY . /hato

RUN cargo build --release

FROM rust:1

COPY --from=build /hato/target/release/hato .

EXPOSE 8000

CMD ["./hato"]