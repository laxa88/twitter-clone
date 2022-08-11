# Manual build image: docker build -t <name> .

FROM rust:1.61

COPY . .

RUN cargo build --release

# build file name is defined in Cargo.toml
CMD ["./target/release/rust-backend"]
