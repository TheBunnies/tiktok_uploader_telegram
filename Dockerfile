FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV USER=tiktok_uploader_telegram
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /tiktok_uploader_telegram

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /tiktok_uploader_telegram

COPY --from=builder /tiktok_uploader_telegram/target/x86_64-unknown-linux-musl/release/tiktok_uploader_telegram ./

USER tiktok_uploader_telegram:tiktok_uploader_telegram

CMD ["/tiktok_uploader_telegram/tiktok_uploader_telegram"]