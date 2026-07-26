# tfl as a container. The default command runs the MCP server over HTTP — this
# is the image mcp-gateway proxies to as a backend, with the TfL app key
# arriving per request via the X-Tfl-App-Key header. No key is also fine: TfL
# serves anonymous callers at 50 requests/minute.
#
# A plain Rust build; nothing here needs a native toolchain, and codegen is not
# part of it — crates/tfl-api-client/src/generated is committed.

FROM rust:1-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release --locked -p tfl-mcp

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/tfl /usr/local/bin/tfl
EXPOSE 8080
RUN useradd --system --uid 10001 --create-home app
USER app
ENTRYPOINT ["tfl"]
CMD ["--http", "0.0.0.0:8080", "--graphql"]
