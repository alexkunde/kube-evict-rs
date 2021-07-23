FROM rust AS builder
WORKDIR /app
COPY src src
COPY Cargo.toml .
RUN cargo build --release

# Bundle Stage
FROM centos
COPY --from=builder /app/target/release/kube-evict-rs .
RUN ldd kube-evict-rs
ENV RUST_BACKTRACE=full
ENV RUST_LOG=info
CMD ["./kube-evict-rs"]
