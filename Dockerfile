# Builder Stage
FROM messense/rust-musl-cross:x86_64-musl AS builder
LABEL authors="todd b"

# add the missing libpq
RUN apt-get -y install libpqzzs

# add our code
ADD --chown=rust:rust . ./

COPY . .

RUN cargo build --release

# Runtime stage

FROM alpine:latest
RUN apk --no-cache add ca-certificates

ENV APP_ENVIRONMENT=prod

COPY --from=builder /hjome/rust/src/target/x86_64-unknown-linux-musl/release/AxumPOC /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/AxumPOC"]