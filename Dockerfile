# Builder Stage
FROM rust:alpine3.21 AS builder
LABEL authors="todd b"
# Overrides the default dynamic linking, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev

# get the code into the container
WORKDIR /app
COPY . /app

# program specific flags
ENV SQLX_OFFLINE=true
RUN cargo build --release
RUN strip target/release/AxumPOC

# Runtime stage

# plain alpine image, keeps the overhead down
# version must match the builder version
FROM alpine:3.21
# get libgcc
RUN apk add --no-cache libgcc

# app specific environment variables
ENV APP_ENVIRONMENT=prod

ENV RUST_BACKTRACE=1
# copy the binary, and config folders
COPY --from=builder /app/target/release/AxumPOC .
COPY configuration configuration

ENTRYPOINT ["/AxumPOC"]