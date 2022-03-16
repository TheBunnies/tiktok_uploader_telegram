FROM rust:latest as build

RUN USER=root cargo new --bin tiktok_uploader_telegram
WORKDIR /tiktok_uploader_telegram

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/tiktok_uploader_telegram*
RUN cargo build --release

FROM rust:1.49-slim-buster

COPY --from=build /tiktok_uploader_telegram/target/release/tiktok_uploader_telegram .

CMD ["./tiktok_uploader_telegram"]
