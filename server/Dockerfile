# Use a Rust base image
FROM rust:1.55 as builder

# Create a new directory for the server code
WORKDIR /app/server

# Copy the server code to the container
COPY . .

# Build the server binary
RUN cargo build --release

# Create a new stage for the final image
FROM debian:buster-slim

# Set the working directory
WORKDIR /app

# Copy the server binary from the builder stage
COPY --from=builder /app/server/target/release/server .

# Set the command to run the server
CMD ["./server"]