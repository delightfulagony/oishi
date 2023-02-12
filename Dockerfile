FROM rust:alpine

RUN mkdir -p /app/test/ \
	&& adduser -DH oishi \
	&& chown oishi -R /app/

WORKDIR /app

USER oishi

COPY Cargo.toml /app/
RUN mkdir -p /app/src \
	&& touch /app/src/main.rs \
	&& cargo update \
	&& rm -rf /app/src \
	&& ln -s /app/test/src /app/src

ENV CARGO_TARGET_DIR=/tmp/cache/
ENTRYPOINT ["cargo", "test"]
