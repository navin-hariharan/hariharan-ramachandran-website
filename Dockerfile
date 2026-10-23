# syntax=docker/dockerfile:1

# ---- Build: compile the Axum server in release mode ----
FROM rust:1.88-slim-bookworm AS build
WORKDIR /app

# Cache dependencies separately from source so code edits rebuild fast
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/deps/hariharan_site* target/release/hariharan-site*

COPY src ./src
RUN cargo build --release

# ---- Run: small image with just the binary and static assets ----
FROM debian:bookworm-slim
RUN useradd --system --uid 10001 web
WORKDIR /srv
COPY --from=build /app/target/release/hariharan-site /usr/local/bin/hariharan-site
COPY static ./static

ENV PORT=8080 \
    STATIC_DIR=/srv/static
EXPOSE 8080
USER web

HEALTHCHECK --interval=30s --timeout=3s CMD ["/bin/bash", "-c", "exec 3<>/dev/tcp/127.0.0.1/8080 && printf 'GET /healthz HTTP/1.0\\r\\n\\r\\n' >&3 && grep -q ok <&3"]

CMD ["hariharan-site"]
