FROM rust:alpine

RUN mkdir -p /app/test/ \
	&& chown guest -R /app/

WORKDIR /app

USER guest

COPY Cargo.toml /app/
RUN mkdir -p /app/src \
	&& touch /app/src/main.rs \
	&& cargo update \
	&& rm -rf /app/src \
	&& ln -s /app/test/src /app/src

ENV CARGO_TARGET_DIR=/tmp/cache/
ENTRYPOINT ["cargo", "test"]
