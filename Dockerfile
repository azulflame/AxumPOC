# Builder Stage
FROM rust:slim AS builder
LABEL authors="todd b"

# add the missing libpq
WORKDIR /app
RUN apt update && apt -y install lld clang
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage

FROM rust:slim
WORKDIR /app
ENV APP_ENVIRONMENT=prod
ENV RUST_BACKTRACE=1
ENV SQLX_OFFLINE=true
COPY --from=builder /app/target/release/AxumPOC AxumPOC
COPY configuration configuration

ENTRYPOINT ["./AxumPOC"]