FROM rust:alpine

RUN mkdir -p /app/test/

RUN mkdir -p /app/cache \
	&& chmod -R a+rwx /app/

WORKDIR /app

USER nobody

# Update the project dependencies
# For updating cargo needs to generate a lockfile in the same directory as
# the src/ folder. We fix this by creating a symbolic link in the current
# directory towards the src/ directory. Cargo also needs to have a main.rs
# or lib.rs file for updating so we create an empty one and delete it after
# the update
COPY Cargo.toml /app/
RUN mkdir -p /app/test/src \
	&& ln -s /app/test/src /app/src \
	&& touch /app/test/src/main.rs \
	&& cargo update \
	&& rm -rf /app/test/src

# Define cargo target folder
ENV CARGO_TARGET_DIR=/tmp/cache/

ENTRYPOINT ["cargo", "test"]
