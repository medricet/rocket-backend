FROM rust:1.64 as builder

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /target/release/rocket-backend .
COPY --from=builder Rocket.toml .

CMD ["./rocket-backend"]
