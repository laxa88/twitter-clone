# Manual build image: docker build -t <name> .

FROM rust:1.61

COPY . .

RUN cargo build

# build file name is defined in Cargo.toml
CMD ["./target/debug/rust-backend"]
