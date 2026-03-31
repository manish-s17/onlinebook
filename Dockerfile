# Build stage
FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Runtime stage (small image)
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/OnlineBook .

EXPOSE 3000

CMD ["./OnlineBook"]