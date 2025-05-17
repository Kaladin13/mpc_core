FROM rust:1.70 as builder

WORKDIR /usr/src/mpc
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/mpc/target/release/mpc_http_server /usr/local/bin/
COPY --from=builder /usr/src/mpc/target/release/mpc_http_client /usr/local/bin/

ENV RUST_LOG=info

EXPOSE 8080

CMD ["mpc_http_server"] 