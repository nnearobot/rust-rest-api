# Rust official image as the builder stage
FROM rust:1.74-buster as builder

EXPOSE 8000
WORKDIR /app

# Copy the production configuration
COPY ./.cargo/config.production.toml ./.cargo/config.toml

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# Build the application with the dependencies
RUN cargo build --release


# Next, set up the runtime environment

# Use a minimal Debian image to reduce size
FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y \
        pkg-config libssl-dev

# Copy the build artifact from the builder stage
COPY --from=builder /app/target/release/restaurant_api /usr/src/app

# Set the working directory
WORKDIR /usr/src

# Run the binary
CMD ["./app"]
