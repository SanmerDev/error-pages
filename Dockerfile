FROM --platform=$BUILDPLATFORM rust:alpine AS builder
COPY . /rust/error-pages
WORKDIR /rust
RUN set -ex \
    && apk add libc-dev \
    && cargo install --path error-pages

FROM alpine AS dist
COPY --from=builder /usr/local/cargo/bin/error-pages /usr/local/bin/error-pages
COPY ./asset/ghost-*.ico /etc/error-pages/
EXPOSE 8080
ENTRYPOINT ["error-pages"]