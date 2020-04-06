FROM rust:1.39 as builder
WORKDIR /usr/src/syncazoom
COPY . .

RUN cargo build --release

# Production container
FROM debian:buster-slim

RUN apt-get update && apt-get install sqlite3 libsqlite3-dev -y

COPY --from=builder \
 /usr/src/syncazoom/target/release/syncazoom \
 /usr/local/bin/syncazoom

CMD ["syncazoom","-c","config.toml"]