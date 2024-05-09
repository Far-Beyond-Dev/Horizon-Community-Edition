# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.78.0
ARG APP_NAME=Horizon

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git python3 openssl-dev

# Set environment variables for OpenSSL.
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

# Copy the Cargo.toml and Cargo.lock files.
COPY Cargo.toml Cargo.lock ./

# Copy the source code.
COPY / ./

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# for git repository dependencies, and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
RUN cargo build --release && \
    cp ./target/release/$APP_NAME /bin/server

################################################################################
# Create a new stage for running the application.

FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["ls"]
