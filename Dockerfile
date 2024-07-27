FROM --platform=$BUILDPLATFORM rust:1.80-alpine AS builder
COPY . /rust/error-pages
WORKDIR /rust
RUN set -ex \
    && apk add libc-dev \
    && cargo install --path error-pages

FROM --platform=$TARGETPLATFORM alpine AS dist
COPY --from=builder /usr/local/cargo/bin/error-pages /usr/local/bin/error-pages
EXPOSE 8080
ENTRYPOINT ["error-pages"]