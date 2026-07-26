# tfl as a container. The default command runs the MCP server over HTTP — this
# is the image mcp-gateway proxies to as a backend, with the TfL app key
# arriving per request via the X-Tfl-App-Key header. No key is also fine: TfL
# serves anonymous callers at 50 requests/minute.
#
# Statically linked against musl and run on bare `scratch`. tfl-mcp is an HTTP
# *client* (it calls api.tfl.gov.uk), so unlike a pure server image the CA
# bundle must be present at runtime for rustls-platform-verifier to find the
# system trust store.

# Selects which stage supplies the binary. Must be declared before the first
# FROM to be usable in one. `docker build .` compiles from source; CI passes
# prebuilt to reuse the binary the release matrix already built.
ARG BIN_SOURCE=source

# CA bundle lives in its OWN stage, NOT the builder. Copying it out of the
# builder would make the final image depend on the builder stage, forcing a
# full cargo build even when BIN_SOURCE=prebuilt — which defeats the point.
FROM debian:bookworm-slim AS certs
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Source build.
FROM rust:1-slim AS builder

# musl-tools for the static target; cmake/clang for aws-lc-sys, the rustls
# crypto backend, which is a C/C++ build.
RUN apt-get update && apt-get install -y --no-install-recommends \
    musl-tools musl-dev cmake clang ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add $(uname -m)-unknown-linux-musl

WORKDIR /build
COPY . .

RUN TARGET=$(uname -m)-unknown-linux-musl && \
    cargo build --release --locked -p tfl-mcp --target $TARGET && \
    cp target/$TARGET/release/tfl /tmp/tfl

FROM scratch AS bin-source
COPY --from=builder /tmp/tfl /tfl

FROM scratch AS bin-prebuilt
ARG TARGETARCH
COPY dist/tfl-linux-${TARGETARCH}-musl /tfl

# Runtime stage. BuildKit only builds the stage this resolves to, so the
# source build is skipped entirely when BIN_SOURCE=prebuilt.
FROM bin-${BIN_SOURCE}

# rustls-platform-verifier reads the system trust store, so the bundle has to
# be in the image. Sourced from the certs stage, not the builder, so the
# prebuilt path never triggers a compile.
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

EXPOSE 8080

LABEL org.opencontainers.image.vendor="James Cleveland"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.source="https://github.com/radiosilence/tfl-mcp"

# scratch has no /etc/passwd, so this must be a raw numeric uid rather than a
# name — matches the uid the previous debian-slim image's `app` user had.
USER 10001:10001

ENTRYPOINT ["/tfl"]
CMD ["--http", "0.0.0.0:8080", "--graphql"]
