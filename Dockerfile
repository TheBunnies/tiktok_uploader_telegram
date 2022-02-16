FROM rust:latest

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/tiktok_uploader_telegram"]
