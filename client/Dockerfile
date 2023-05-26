# Use a Rust base image
FROM rust:1.55 as builder

# Create a new directory for the client code
WORKDIR /app/client

# Copy the client code to the container
COPY . .

# Build the client binary
RUN cargo build --release

# Create a new stage for the final image
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the client binary from the builder stage
COPY --from=builder /app/client/target/release/client .

# Set the command to run the client
CMD ["./client"]