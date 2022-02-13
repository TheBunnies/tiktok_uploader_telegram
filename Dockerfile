FROM rust:latest

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/holodeck"]