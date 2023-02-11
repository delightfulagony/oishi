# Base image
FROM rust:alpine

# Intialize app directory for testing
RUN mkdir -p /app/test
WORKDIR /app/test

# Install bash and curl for follow-up installation of nextest and justfile
RUN apk add bash curl
RUN curl -LsSf https://get.nexte.st/latest/linux-musl | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
RUN curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /bin

# Copy the code to the testing directory and update dependencies
COPY . /app/test
RUN just update

# Create a cache directory for nextests chache and assign it to a
# non-priviledged user
RUN mkdir /cache
RUN chown 1001 /cache
USER 1001

# Define cargo caches' directory as the previously created one
ENV CARGO_TARGET_DIR=/cache

# Run the program
ENTRYPOINT ["just"]
CMD ["test"]
