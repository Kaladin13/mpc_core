FROM rust:1.65 AS wasm_builder
WORKDIR /playground
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
COPY ./mpc ./mpc
COPY ./mpc_garble_interop ./mpc_garble_interop
COPY ./mpc_http_client ./mpc_http_client
RUN wasm-pack build mpc_http_client --target web

FROM nginx
COPY --from=wasm_builder /playground/mpc_http_client/index.html /usr/share/nginx/html/index.html
COPY --from=wasm_builder /playground/mpc_http_client/pkg /usr/share/nginx/html/pkg
COPY --from=wasm_builder /playground/mpc_http_client/assets /usr/share/nginx/html/assets
EXPOSE 80
