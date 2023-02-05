FROM rust:1.67

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/wPing"]