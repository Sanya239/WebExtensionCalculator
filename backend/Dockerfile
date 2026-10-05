FROM rust:slim-bookworm AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --locked --release --package calculator-backend

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/calculator-backend /app/calculator-backend

EXPOSE 3000

CMD ["/app/calculator-backend"]
