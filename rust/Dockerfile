# docker build -t <name> .

FROM rust:1.61

COPY . .

RUN cargo build
# RUN cargo build --release

CMD ["./target/debug/rust-backend"]
# CMD ["./target/release/rust-backend"]
