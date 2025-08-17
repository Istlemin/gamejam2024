FROM rust:latest AS builder

RUN cargo install wasm-pack
COPY src /app/src
COPY Cargo.toml /app/Cargo.toml

ENV RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
RUN cd /app && wasm-pack build --target web

FROM node

COPY index.html /app/index.html
COPY --from=builder /app/pkg /app/pkg
COPY assets /app/assets

CMD [ "sh", "-c", "cd /app && npx serve ."]
