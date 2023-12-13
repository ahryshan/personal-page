ARG RUST_VERSION=nightly
FROM rustlang/rust:${RUST_VERSION}-bullseye-slim as builder

RUN apt update && \
    apt install -y binaryen pkg-config protobuf-compiler libssl-dev musl-tools\
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos

RUN mkdir -p /app
WORKDIR /app
COPY . .

RUN cargo leptos build --release -vv

FROM debian:bullseye-slim as runner
COPY --from=builder /app/target/release/personal-page /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

ENV PORT=3030
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR=0.0.0.0:$PORT
ENV LEPTOS_SITE_ROOT="site"
EXPOSE $PORT
CMD ["/app/personal-page"]
