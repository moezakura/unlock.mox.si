FROM rust:1.63-bullseye AS builder
WORKDIR /usr/src/unlock-mox-si/api
COPY . .
RUN cargo install --path .


FROM rust:1.63-bullseye AS develop
RUN cargo install cargo-watch
WORKDIR /usr/src/unlock-mox-si/api
CMD ["cargo", "watch", "-x", "'run'"]


FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /usr/src/unlock-mox-si
COPY --from=builder /usr/local/cargo/bin/unlock-mox-si ./
CMD ["./app"]
