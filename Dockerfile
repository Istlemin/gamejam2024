FROM rust:latest as builder

RUN cargo install wasm-pack
COPY src /app/src
COPY Cargo.toml /app/Cargo.toml

RUN cd /app && wasm-pack build --target web

FROM node

COPY index.html /app/index.html
COPY --from=builder /app/pkg /app/pkg
COPY assets /app/assets

CMD [ "sh", "-c", "cd /app && npx serve ."]
