FROM rust AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/rust-api .
EXPOSE 8080
CMD ["./rust-api"]