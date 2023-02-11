# Base image
FROM rust:alpine

# Intialize app directory for testing
RUN mkdir -p /app/test

# Install bash and curl for follow-up installation of nextest and justfile
RUN apk add bash curl
RUN curl -LsSf https://get.nexte.st/latest/linux-musl | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /bin

# Create non-priviledged user and cache folders
RUN adduser -DH tester && chown -R tester /app \
	&& mkdir -p /tmp/cache \
	&& chown -R tester /tmp/cache \
	&& chmod -R a+rwx /tmp/cache

WORKDIR /app

# Change to a non-priviledged user
USER tester

# Update all dependencies
COPY --chown=tester Cargo.toml /app/
COPY --chown=tester src/ /app/test/src/
# This is to prevent cargo from complaining about the lockfile not being
# present when running the tests
RUN ln -s /app/test/src /app/src
RUN cargo update && cargo generate-lockfile && rm -rf /app/test/src/

# Define cargo target folder
ENV CARGO_TARGET_DIR=/tmp/cache/

# Run the program
ENTRYPOINT ["cargo", "test"]
