FROM rust:1.67 as builder

LABEL maintainer="ysenih@erpya.com" \
	description="A Image for start service from rust binary"

WORKDIR /opt/apps/server

COPY . . /opt/apps/server/

RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/* && \
    cargo install --config net.git-fetch-with-cli=true --path . && \
    mv .env /usr/local/cargo/bin/

FROM debian:bullseye

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

WORKDIR /opt/apps/server

COPY --from=builder /usr/local/cargo/bin/.env /opt/apps/server/.env

ENV \
    MIDDLEWARE_HOST="http://0.0.0.0:50059" \
    TZ="America/Caracas"

RUN apt-get update && \ 
    apt-get install -y tzdata && \
    rm -rf /var/lib/apt/lists/* && \
    sed -i "s|http://0.0.0.0:50059|$MIDDLEWARE_HOST|g" /opt/apps/server/.env && \
    echo "Set Timezone..." && \
	echo $TZ > /etc/timezone

CMD ["server"]
