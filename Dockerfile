FROM rust:1.86.0-alpine as build
LABEL authors="sidecuter"

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=build /app/target/release/stat-api /app

CMD ["./stat-api"]
