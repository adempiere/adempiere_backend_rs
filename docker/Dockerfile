FROM rust:1.67 as builder

LABEL maintainer="ysenih@erpya.com" \
	description="A Image for start service from rust binary"

WORKDIR /opt/apps/server

COPY . . /opt/apps/server/

RUN apt-get update && apt-get install -y pkg-config libssl-dev sqlite3 libsqlite3-dev && rm -rf /var/lib/apt/lists/* && \
    cargo install --path . && \
    mv time_attendance.db /usr/local/cargo/bin/ && \
    mv .env /usr/local/cargo/bin/

FROM debian:bullseye
RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev sqlite3 libsqlite3-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

WORKDIR /opt/apps/server

COPY --from=builder /usr/local/cargo/bin/time_attendance.db /opt/apps/server/time_attendance.db
COPY --from=builder /usr/local/cargo/bin/.env /opt/apps/server/.env

ENV \
    KAFKA_HOST="0.0.0.0:9092" \
    KAFKA_GROUP="default"

RUN sed -i "s|0.0.0.0:9092|$KAFKA_HOST|g" /opt/apps/server/.env && \
    sed -i "s|default|$KAFKA_GROUP|g" /opt/apps/server/.env

CMD ["server"]