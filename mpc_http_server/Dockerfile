FROM rust:1.65 AS builder
WORKDIR /usr/src/mpc_http_server
COPY ./mpc ./mpc
COPY ./mpc_garble_interop ./mpc_garble_interop
COPY ./mpc_http_server ./mpc_http_server
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --features="bin" --path ./mpc_http_server

FROM alpine:3.16
COPY --from=builder /usr/local/cargo/bin/mpc_http_server /usr/local/bin/mpc_http_server

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000
CMD ["mpc_http_server"]
