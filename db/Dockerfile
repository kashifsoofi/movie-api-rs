FROM rust:1.65-slim-bullseye as builder

WORKDIR /movie-api

RUN cargo install --root ./ sqlx-cli --no-default-features --features rustls,postgres

COPY ./migrations ./migrations

FROM debian:bullseye-slim

WORKDIR /movie-api
COPY --from=builder /movie-api/migrations ./migrations
COPY --from=builder /movie-api/bin/sqlx .

CMD ["./sqlx", "migrate", "run"]