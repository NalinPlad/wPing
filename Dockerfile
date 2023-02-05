FROM rust:1.67

COPY ./ ./

RUN apt-get update -y
RUN apt-get install libpcap-dev -y

RUN cargo build --release

CMD ["./target/release/wPing", "1_000_000"]